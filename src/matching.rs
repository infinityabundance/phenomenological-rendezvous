//! Pattern matching and rendezvous logic.

/// Placeholder matcher configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct Matcher {
    _private: (),
}

impl Matcher {
    /// Create a matcher placeholder.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
