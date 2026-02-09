//! Submodality pattern definitions and helpers.

/// Placeholder pattern type.
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    _private: (),
}

impl Pattern {
    /// Create an empty pattern placeholder.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
