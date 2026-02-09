//! Submodality pattern definitions and helpers.

use serde::{Deserialize, Serialize};

pub const BRIGHTNESS_MIN: f32 = 0.0;
pub const BRIGHTNESS_MAX: f32 = 1.0;
pub const COLOR_TEMP_MIN: f32 = 2000.0;
pub const COLOR_TEMP_MAX: f32 = 10_000.0;
pub const FOCAL_DISTANCE_MIN: f32 = 0.0;
pub const FOCAL_DISTANCE_MAX: f32 = 1.0;
pub const VOLUME_MIN: f32 = 0.0;
pub const VOLUME_MAX: f32 = 1.0;
pub const TEMPO_MIN: f32 = 0.0;
pub const TEMPO_MAX: f32 = 300.0;
pub const PITCH_MIN: f32 = 20.0;
pub const PITCH_MAX: f32 = 20_000.0;
pub const TEMPERATURE_MIN: f32 = 10.0;
pub const TEMPERATURE_MAX: f32 = 40.0;
pub const MOVEMENT_MIN: f32 = 0.0;
pub const MOVEMENT_MAX: f32 = 1.0;
pub const AROUSAL_MIN: f32 = 0.0;
pub const AROUSAL_MAX: f32 = 1.0;

/// A submodality pattern as described in the paper.
///
/// This mirrors the SubmodalityPattern pseudo-code and keeps raw values in
/// their natural units. Normalization to `[0, 1]` is handled separately.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmodalityPattern {
    /// Brightness, normalized to `[0.0, 1.0]`.
    pub brightness: f32,
    /// Color temperature in Kelvin (2000–10000).
    pub color_temp: f32,
    /// Focal distance, normalized to `[0.0, 1.0]`.
    pub focal_distance: f32,
    /// Volume, normalized to `[0.0, 1.0]`.
    pub volume: f32,
    /// Tempo in BPM (0–300).
    pub tempo: f32,
    /// Pitch in Hertz (20–20000).
    pub pitch: f32,
    /// Temperature in Celsius.
    pub temperature: f32,
    /// Movement, normalized to `[0.0, 1.0]`.
    pub movement: f32,
    /// Arousal, normalized to `[0.0, 1.0]`.
    pub arousal: f32,
}

impl SubmodalityPattern {
    /// Create a neutral baseline pattern for initialization and testing.
    ///
    /// "Neutral" means unit-range fields are centered or zeroed, and absolute
    /// scale fields are set to commonly used midpoints. This is a placeholder
    /// baseline and should be replaced with domain-specific defaults later.
    pub fn zeros() -> Self {
        Self {
            brightness: 0.5,
            color_temp: 6500.0,
            focal_distance: 0.5,
            volume: 0.5,
            tempo: 0.0,
            pitch: 440.0,
            temperature: 20.0,
            movement: 0.0,
            arousal: 0.0,
        }
    }

    /// Normalize this pattern into `[0, 1]` ranges for distance calculations.
    ///
    /// Temperature normalization assumes a `10..=40` Celsius operating window
    /// as a placeholder until domain-specific bounds are defined.
    pub fn normalize(&self) -> NormalizedPattern {
        NormalizedPattern {
            brightness: clamp01(self.brightness),
            color_temp: clamp01((self.color_temp - COLOR_TEMP_MIN) / (COLOR_TEMP_MAX - COLOR_TEMP_MIN)),
            focal_distance: clamp01(self.focal_distance),
            volume: clamp01(self.volume),
            tempo: clamp01(self.tempo / TEMPO_MAX),
            pitch: clamp01((self.pitch - PITCH_MIN) / (PITCH_MAX - PITCH_MIN)),
            temperature: clamp01((self.temperature - TEMPERATURE_MIN) / (TEMPERATURE_MAX - TEMPERATURE_MIN)),
            movement: clamp01(self.movement),
            arousal: clamp01(self.arousal),
        }
    }
}

/// A fully normalized submodality pattern with values in `[0, 1]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedPattern {
    pub brightness: f32,
    pub color_temp: f32,
    pub focal_distance: f32,
    pub volume: f32,
    pub tempo: f32,
    pub pitch: f32,
    pub temperature: f32,
    pub movement: f32,
    pub arousal: f32,
}

fn clamp01(value: f32) -> f32 {
    if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

/// Map a 16-bit integer into a floating-point range `[min, max]`.
pub fn quantize_u16_to_range(val: u16, min: f32, max: f32) -> f32 {
    let fraction = f32::from(val) / f32::from(u16::MAX);
    min + (max - min) * fraction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_json_round_trip() {
        let pattern = SubmodalityPattern::zeros();
        let json = serde_json::to_string(&pattern).expect("serialize");
        let decoded: SubmodalityPattern = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(pattern, decoded);
    }
}
