//! Simulation tools for testing rendezvous dynamics.

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::matching::{MatchingConfig, Matcher};
use crate::pattern::{
    SubmodalityPattern, AROUSAL_MAX, AROUSAL_MIN, BRIGHTNESS_MAX, BRIGHTNESS_MIN, COLOR_TEMP_MAX,
    COLOR_TEMP_MIN, FOCAL_DISTANCE_MAX, FOCAL_DISTANCE_MIN, MOVEMENT_MAX, MOVEMENT_MIN, PITCH_MAX,
    PITCH_MIN, TEMPERATURE_MAX, TEMPERATURE_MIN, TEMPO_MAX, TEMPO_MIN, VOLUME_MAX, VOLUME_MIN,
};
use crate::srt::{pattern_from_srt, SemanticRendezvousToken};

/// Configuration for rendezvous simulations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub num_peers: usize,
    pub num_trials: usize,
    pub epsilon: f32,
    pub window_size: usize,
    pub apply_geo_filter: bool,
    /// Factor to reduce candidate pool size (e.g. 1e6).
    pub geo_filter_factor: f32,
}

/// Output metrics from a simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub total_trials: usize,
    pub total_peer_samples: usize,
    pub single_match_count: usize,
    pub double_match_count: usize,
    pub single_match_probability: f64,
    pub double_match_probability: f64,
    pub effective_peer_count: f64,
    pub expected_matches_in_pool: f64,
    pub pool_match_probability: f64,
}

/// Generate a random submodality pattern using uniform sampling per dimension.
///
/// This assumes independence and uniform distributions across the allowed
/// ranges. These assumptions are for exploration only and do not reflect real
/// sensor distributions.
pub fn random_pattern<R: Rng + ?Sized>(rng: &mut R) -> SubmodalityPattern {
    SubmodalityPattern {
        brightness: rng.gen_range(BRIGHTNESS_MIN..=BRIGHTNESS_MAX),
        color_temp: rng.gen_range(COLOR_TEMP_MIN..=COLOR_TEMP_MAX),
        focal_distance: rng.gen_range(FOCAL_DISTANCE_MIN..=FOCAL_DISTANCE_MAX),
        volume: rng.gen_range(VOLUME_MIN..=VOLUME_MAX),
        tempo: rng.gen_range(TEMPO_MIN..=TEMPO_MAX),
        pitch: rng.gen_range(PITCH_MIN..=PITCH_MAX),
        temperature: rng.gen_range(TEMPERATURE_MIN..=TEMPERATURE_MAX),
        movement: rng.gen_range(MOVEMENT_MIN..=MOVEMENT_MAX),
        arousal: rng.gen_range(AROUSAL_MIN..=AROUSAL_MAX),
    }
}

fn matches_target(
    measured: &SubmodalityPattern,
    target: &SubmodalityPattern,
    epsilon: f32,
    window_size: usize,
) -> bool {
    let mut matcher = Matcher::new(MatchingConfig::new(epsilon, window_size));
    for _ in 0..window_size.max(1) {
        if matcher.observe(measured, target) {
            return true;
        }
    }
    false
}

/// Run a simulation to estimate collision and false rendezvous rates.
pub fn run_simulation(
    config: &SimulationConfig,
    srt: &SemanticRendezvousToken,
    salt: &[u8],
) -> SimulationResult {
    let target = pattern_from_srt(srt, salt);
    let mut rng = rand::thread_rng();

    let mut single_match_count = 0usize;
    let mut double_match_count = 0usize;
    let mut total_peer_samples = 0usize;

    for _ in 0..config.num_trials {
        for _ in 0..config.num_peers {
            let peer = random_pattern(&mut rng);
            if matches_target(&peer, &target, config.epsilon, config.window_size) {
                single_match_count += 1;
            }
            total_peer_samples += 1;
        }

        let peer_a = random_pattern(&mut rng);
        let peer_b = random_pattern(&mut rng);
        if matches_target(&peer_a, &target, config.epsilon, config.window_size)
            && matches_target(&peer_b, &target, config.epsilon, config.window_size)
        {
            double_match_count += 1;
        }
    }

    let single_match_probability =
        (single_match_count as f64) / (total_peer_samples.max(1) as f64);
    let double_match_probability =
        (double_match_count as f64) / (config.num_trials.max(1) as f64);

    let effective_peer_count = if config.apply_geo_filter && config.geo_filter_factor > 0.0 {
        (config.num_peers as f64 / config.geo_filter_factor as f64).max(1.0)
    } else {
        config.num_peers as f64
    };

    let expected_matches_in_pool = single_match_probability * effective_peer_count;
    let pool_match_probability =
        1.0 - (1.0 - single_match_probability).powf(effective_peer_count);

    SimulationResult {
        total_trials: config.num_trials,
        total_peer_samples,
        single_match_count,
        double_match_count,
        single_match_probability,
        double_match_probability,
        effective_peer_count,
        expected_matches_in_pool,
        pool_match_probability,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulation_runs_with_small_config() {
        let config = SimulationConfig {
            num_peers: 100,
            num_trials: 100,
            epsilon: 0.2,
            window_size: 1,
            apply_geo_filter: false,
            geo_filter_factor: 1e6,
        };
        let srt = SemanticRendezvousToken::from_bytes([1u8; 32]);
        let result = run_simulation(&config, &srt, b"salt");

        assert_eq!(result.total_trials, 100);
        assert!(result.single_match_probability >= 0.0);
        assert!(result.single_match_probability <= 1.0);
        assert!(result.double_match_probability >= 0.0);
        assert!(result.double_match_probability <= 1.0);
    }
}
