//! Pattern matching and rendezvous logic.

use crate::pattern::{NormalizedPattern, SubmodalityPattern};

/// Compute Euclidean distance in normalized 9D submodality space.
///
/// Inputs must already be normalized to `[0, 1]` ranges.
pub fn euclidean_distance(a: &NormalizedPattern, b: &NormalizedPattern) -> f32 {
    let mut sum = 0.0;
    sum += (a.brightness - b.brightness).powi(2);
    sum += (a.color_temp - b.color_temp).powi(2);
    sum += (a.focal_distance - b.focal_distance).powi(2);
    sum += (a.volume - b.volume).powi(2);
    sum += (a.tempo - b.tempo).powi(2);
    sum += (a.pitch - b.pitch).powi(2);
    sum += (a.temperature - b.temperature).powi(2);
    sum += (a.movement - b.movement).powi(2);
    sum += (a.arousal - b.arousal).powi(2);
    sum.sqrt()
}

/// Configuration for matching behavior.
///
/// Assumes a static epsilon and a fixed temporal window, which are simple
/// baselines meant for experimentation rather than adaptive production use.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatchingConfig {
    /// Matching threshold in normalized 9D space.
    pub epsilon: f32,
    /// Number of consecutive observations required within `epsilon`.
    pub window_size: usize,
}

impl MatchingConfig {
    /// Create a config with an epsilon and smoothing window size.
    pub fn new(epsilon: f32, window_size: usize) -> Self {
        Self {
            epsilon,
            window_size,
        }
    }
}

/// Matcher that performs temporal smoothing over recent observations.
///
/// This matcher assumes measured patterns arrive as a time-ordered stream and
/// that each observation is comparable to the target pattern without additional
/// context such as sensor calibration or quality scores.
#[derive(Debug, Clone)]
pub struct Matcher {
    /// Matching behavior configuration.
    config: MatchingConfig,
    /// Sliding window of recent match results.
    window: Vec<bool>,
}

impl Matcher {
    /// Create a matcher with the provided configuration.
    pub fn new(config: MatchingConfig) -> Self {
        Self {
            config,
            window: Vec::with_capacity(config.window_size),
        }
    }

    /// Observe a new measurement and return whether a match is stable.
    ///
    /// This normalizes both patterns, computes distance, and records whether
    /// the distance is within `epsilon`. It returns `true` only when the most
    /// recent `window_size` observations are all within `epsilon`.
    pub fn observe(
        &mut self,
        measured: &SubmodalityPattern,
        target: &SubmodalityPattern,
    ) -> bool {
        let measured_norm = measured.normalize();
        let target_norm = target.normalize();
        let distance = euclidean_distance(&measured_norm, &target_norm);
        let within = distance <= self.config.epsilon;

        if self.config.window_size == 0 {
            return within;
        }

        if self.window.len() == self.config.window_size {
            self.window.remove(0);
        }
        self.window.push(within);

        self.window.len() == self.config.window_size && self.window.iter().all(|v| *v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::{
        SubmodalityPattern, AROUSAL_MAX, AROUSAL_MIN, BRIGHTNESS_MAX, BRIGHTNESS_MIN,
        COLOR_TEMP_MAX, COLOR_TEMP_MIN, FOCAL_DISTANCE_MAX, FOCAL_DISTANCE_MIN, MOVEMENT_MAX,
        MOVEMENT_MIN, PITCH_MAX, PITCH_MIN, TEMPERATURE_MAX, TEMPERATURE_MIN, TEMPO_MAX, TEMPO_MIN,
        VOLUME_MAX, VOLUME_MIN,
    };

    fn min_pattern() -> SubmodalityPattern {
        SubmodalityPattern {
            brightness: BRIGHTNESS_MIN,
            color_temp: COLOR_TEMP_MIN,
            focal_distance: FOCAL_DISTANCE_MIN,
            volume: VOLUME_MIN,
            tempo: TEMPO_MIN,
            pitch: PITCH_MIN,
            temperature: TEMPERATURE_MIN,
            movement: MOVEMENT_MIN,
            arousal: AROUSAL_MIN,
        }
    }

    fn max_pattern() -> SubmodalityPattern {
        SubmodalityPattern {
            brightness: BRIGHTNESS_MAX,
            color_temp: COLOR_TEMP_MAX,
            focal_distance: FOCAL_DISTANCE_MAX,
            volume: VOLUME_MAX,
            tempo: TEMPO_MAX,
            pitch: PITCH_MAX,
            temperature: TEMPERATURE_MAX,
            movement: MOVEMENT_MAX,
            arousal: AROUSAL_MAX,
        }
    }

    #[test]
    fn patterns_far_apart_never_match() {
        let config = MatchingConfig::new(0.1, 3);
        let mut matcher = Matcher::new(config);
        let measured = min_pattern();
        let target = max_pattern();

        for _ in 0..5 {
            assert!(!matcher.observe(&measured, &target));
        }
    }

    #[test]
    fn patterns_match_after_window_size_observations() {
        let config = MatchingConfig::new(0.05, 3);
        let mut matcher = Matcher::new(config);
        let measured = SubmodalityPattern::zeros();
        let target = SubmodalityPattern::zeros();

        assert!(!matcher.observe(&measured, &target));
        assert!(!matcher.observe(&measured, &target));
        assert!(matcher.observe(&measured, &target));
    }

    #[test]
    fn epsilon_affects_match_behavior() {
        let measured = SubmodalityPattern::zeros();
        let mut target = SubmodalityPattern::zeros();
        target.brightness = BRIGHTNESS_MAX;

        let mut strict = Matcher::new(MatchingConfig::new(0.01, 1));
        let mut loose = Matcher::new(MatchingConfig::new(1.5, 1));

        assert!(!strict.observe(&measured, &target));
        assert!(loose.observe(&measured, &target));
    }
}
