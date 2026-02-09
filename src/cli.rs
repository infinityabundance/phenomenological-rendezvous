//! CLI scaffolding for future binaries.

/// Placeholder CLI settings.
#[derive(Debug, Clone, PartialEq)]
pub struct Cli {
    _private: (),
}

impl Cli {
    /// Create a CLI placeholder.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
