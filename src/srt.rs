//! SRT (Symbolic Resonance Token) encoding primitives.

/// Placeholder SRT type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Srt {
    _private: (),
}

impl Srt {
    /// Create an empty SRT placeholder.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
