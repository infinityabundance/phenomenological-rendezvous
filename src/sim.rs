//! Simulation tools for testing rendezvous dynamics.

/// Placeholder simulator.
#[derive(Debug, Clone, PartialEq)]
pub struct Simulator {
    _private: (),
}

impl Simulator {
    /// Create a simulator placeholder.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
