//! Reference implementation of the Phenomenological Rendezvous protocol.
//!
//! This crate provides SRT encoding, submodality patterns, matching logic,
//! and simulation tools.

pub mod srt;
pub mod pattern;
pub mod matching;
pub mod sim;

pub use pattern::{NormalizedPattern, SubmodalityPattern};
pub use srt::SemanticRendezvousToken;
