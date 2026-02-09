# Code Structure

## Overview
This codebase provides a Rust reference implementation of the Phenomenological Rendezvous protocol, with a focus on protocol logic, data structures, and simulation scaffolding.

## Modules
- `srt` - Symbolic Resonance Token encoding primitives.
- `pattern` - Submodality pattern definitions and helpers.
- `matching` - Pattern matching and rendezvous logic.
- `sim` - Simulation tools for testing rendezvous dynamics.
- `cli` - CLI scaffolding for future binaries.

## Matching Protocol
`NormalizedPattern` is the normalized representation of a `SubmodalityPattern` with all fields mapped into `[0, 1]`.

`euclidean_distance` computes distance in the normalized 9D space.

`Matcher` applies a configurable `epsilon` threshold and a `window_size` smoothing rule. A match is reported only when the most recent `window_size` observations are all within `epsilon`.

## Out of Scope
This project does not implement real biosensor integrations, device firmware, or mobile apps. It focuses strictly on the protocol and its logic.
