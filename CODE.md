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

## Simulation
The simulation module generates random `SubmodalityPattern` instances using uniform, independent distributions across each dimension's allowed range. These assumptions are for exploration only and are not intended as a security proof or a faithful model of real sensor distributions.

`run_simulation` estimates match probabilities for a single random peer and for two independent peers matching the same SRT. An optional geographic filter factor reduces the effective peer pool size when approximating false rendezvous rates.

## Out of Scope
This project does not implement real biosensor integrations, device firmware, or mobile apps. It focuses strictly on the protocol and its logic.
