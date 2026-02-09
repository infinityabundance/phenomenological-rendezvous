//! Semantic Rendezvous Token (SRT) encoding primitives.

use std::fmt;
use std::str::FromStr;

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::pattern::{
    quantize_u16_to_range, SubmodalityPattern, AROUSAL_MAX, AROUSAL_MIN, BRIGHTNESS_MAX,
    BRIGHTNESS_MIN, COLOR_TEMP_MAX, COLOR_TEMP_MIN, FOCAL_DISTANCE_MAX, FOCAL_DISTANCE_MIN,
    MOVEMENT_MAX, MOVEMENT_MIN, PITCH_MAX, PITCH_MIN, TEMPERATURE_MAX, TEMPERATURE_MIN, TEMPO_MAX,
    TEMPO_MIN, VOLUME_MAX, VOLUME_MIN,
};

/// A Semantic Rendezvous Token (SRT).
///
/// An SRT is a shared secret key used for HMAC-based derivation of target
/// patterns during rendezvous. We treat it as an opaque 32-byte value and do
/// not attempt to derive it from passwords or other human inputs here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRendezvousToken([u8; 32]);

impl SemanticRendezvousToken {
    /// Create an SRT from raw 32-byte input.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Create an SRT from a byte slice.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, SrtParseError> {
        if bytes.len() != 32 {
            return Err(SrtParseError::InvalidLength(bytes.len()));
        }
        let mut raw = [0u8; 32];
        raw.copy_from_slice(bytes);
        Ok(Self(raw))
    }

    /// Borrow the underlying bytes.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Parse an SRT from a hex string.
    pub fn from_hex(hex: &str) -> Result<Self, SrtParseError> {
        hex.parse()
    }
}

/// Derive a `SubmodalityPattern` from an SRT and salt (oracle-state).
///
/// This uses HMAC-SHA256 with the SRT as key and `salt` as the message.
/// The resulting 32-byte digest is partitioned into 16-bit chunks:
///
/// - `digest[0..2]`  -> brightness
/// - `digest[2..4]`  -> color_temp
/// - `digest[4..6]`  -> focal_distance
/// - `digest[6..8]`  -> volume
/// - `digest[8..10]` -> tempo
/// - `digest[10..12]` -> pitch
/// - `digest[12..14]` -> temperature
/// - `digest[14..16]` -> movement
/// - `digest[16..18]` -> arousal
///
/// Remaining bytes are reserved for future extensions.
pub fn pattern_from_srt(
    srt: &SemanticRendezvousToken,
    salt: &[u8],
) -> SubmodalityPattern {
    let mut mac = Hmac::<Sha256>::new_from_slice(srt.as_bytes())
        .expect("HMAC can take a 32-byte key");
    mac.update(salt);
    let digest = mac.finalize().into_bytes();

    let mut read = |start: usize| -> u16 {
        let hi = digest[start] as u16;
        let lo = digest[start + 1] as u16;
        (hi << 8) | lo
    };

    SubmodalityPattern {
        brightness: quantize_u16_to_range(read(0), BRIGHTNESS_MIN, BRIGHTNESS_MAX),
        color_temp: quantize_u16_to_range(read(2), COLOR_TEMP_MIN, COLOR_TEMP_MAX),
        focal_distance: quantize_u16_to_range(read(4), FOCAL_DISTANCE_MIN, FOCAL_DISTANCE_MAX),
        volume: quantize_u16_to_range(read(6), VOLUME_MIN, VOLUME_MAX),
        tempo: quantize_u16_to_range(read(8), TEMPO_MIN, TEMPO_MAX),
        pitch: quantize_u16_to_range(read(10), PITCH_MIN, PITCH_MAX),
        temperature: quantize_u16_to_range(read(12), TEMPERATURE_MIN, TEMPERATURE_MAX),
        movement: quantize_u16_to_range(read(14), MOVEMENT_MIN, MOVEMENT_MAX),
        arousal: quantize_u16_to_range(read(16), AROUSAL_MIN, AROUSAL_MAX),
    }
}

impl fmt::Display for SemanticRendezvousToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl FromStr for SemanticRendezvousToken {
    type Err = SrtParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 64 {
            return Err(SrtParseError::InvalidHexLength(s.len()));
        }
        let mut bytes = [0u8; 32];
        for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
            let hi = decode_hex_nibble(chunk[0])?;
            let lo = decode_hex_nibble(chunk[1])?;
            bytes[i] = (hi << 4) | lo;
        }
        Ok(Self(bytes))
    }
}

fn decode_hex_nibble(byte: u8) -> Result<u8, SrtParseError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(SrtParseError::InvalidHexCharacter(byte as char)),
    }
}

/// Errors returned when parsing SRTs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SrtParseError {
    /// The input hex string did not have 64 characters.
    InvalidHexLength(usize),
    /// The input byte slice was not 32 bytes long.
    InvalidLength(usize),
    /// The input included a non-hex character.
    InvalidHexCharacter(char),
}

impl fmt::Display for SrtParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHexLength(len) => write!(f, "expected 64 hex chars, got {len}"),
            Self::InvalidLength(len) => write!(f, "expected 32 bytes, got {len}"),
            Self::InvalidHexCharacter(ch) => write!(f, "invalid hex character '{ch}'"),
        }
    }
}

impl std::error::Error for SrtParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srt_hex_round_trip() {
        let bytes: [u8; 32] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let srt = SemanticRendezvousToken::from_bytes(bytes);
        let encoded = srt.to_string();
        let decoded: SemanticRendezvousToken = encoded.parse().expect("parse hex");
        assert_eq!(srt, decoded);
    }

    #[test]
    fn srt_pattern_is_deterministic() {
        let srt = SemanticRendezvousToken::from_bytes([7u8; 32]);
        let salt = b"oracle-state";
        let first = pattern_from_srt(&srt, salt);
        let second = pattern_from_srt(&srt, salt);
        assert_eq!(first, second);
    }

    #[test]
    fn srt_pattern_changes_with_salt() {
        let srt = SemanticRendezvousToken::from_bytes([9u8; 32]);
        let base = pattern_from_srt(&srt, b"salt-a");
        let mut different = 0;
        for salt in [b"salt-b", b"salt-c", b"salt-d", b"salt-e"] {
            let candidate = pattern_from_srt(&srt, salt);
            if candidate != base {
                different += 1;
            }
        }
        assert!(different >= 2);
    }
}
