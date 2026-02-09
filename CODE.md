# Code Structure

## Overview
This codebase provides a Rust reference implementation of the Phenomenological Rendezvous protocol from the paper (DOI: 10.5281/zenodo.18558066). It focuses on protocol logic, data structures, and tooling that let researchers reproduce the paper's core mechanics without building a full device stack.

At a high level, an SRT (a shared secret) is mapped into a target submodality pattern. Measured patterns are compared in normalized space using a configurable threshold and a temporal stability window. The intent is to study when peers can rendezvous without centralized coordination.

The threat model is intentionally conservative and high level. The implementation assumes SRTs are shared out of band and treats the derived patterns as opaque outputs; it does not attempt to model sensor spoofing, active adversaries, or side channels. The code here is a reference baseline rather than a security proof.

## Modules
`srt`
Responsibilities: Represent SRTs, parse/format hex, derive target patterns from SRT + salt.
Key types and functions: `SemanticRendezvousToken`, `pattern_from_srt`.
Typical call flow: Parse or construct an SRT, then call `pattern_from_srt` with an oracle-state to get a target pattern.

`pattern`
Responsibilities: Define raw and normalized submodality patterns and range helpers.
Key types and functions: `SubmodalityPattern`, `NormalizedPattern`, `quantize_u16_to_range`, range constants.
Typical call flow: Construct or deserialize a `SubmodalityPattern`, normalize it, and feed it into matching or simulation.

`matching`
Responsibilities: Compute distances in normalized space and apply temporal smoothing.
Key types and functions: `euclidean_distance`, `MatchingConfig`, `Matcher`.
Typical call flow: Normalize measured and target patterns, compute distance, and track consecutive matches through `Matcher::observe`.

`sim`
Responsibilities: Generate random patterns and estimate collision/false rendezvous rates.
Key types and functions: `SimulationConfig`, `SimulationResult`, `run_simulation`, `random_pattern`.
Typical call flow: Configure simulation parameters, derive a target pattern from an SRT, then run Monte Carlo trials.

`cli`
Responsibilities: Provide offline command-line tooling around the core library.
Key types and functions: `CliArgs`, `Commands`, `run`.
Typical call flow: Parse CLI args, call library functions (encoding, matching, simulation), emit JSON results.

## Matching Protocol
`NormalizedPattern` is the normalized representation of a `SubmodalityPattern` with all fields mapped into `[0, 1]`.

`euclidean_distance` computes distance in the normalized 9D space.

`Matcher` applies a configurable `epsilon` threshold and a `window_size` smoothing rule. A match is reported only when the most recent `window_size` observations are all within `epsilon`.

## Simulation
The simulation module generates random `SubmodalityPattern` instances using uniform, independent distributions across each dimension's allowed range. These assumptions are for exploration only and are not intended as a security proof or a faithful model of real sensor distributions.

`run_simulation` estimates match probabilities for a single random peer and for two independent peers matching the same SRT. An optional geographic filter factor reduces the effective peer pool size when approximating false rendezvous rates.

## Design Decisions
Euclidean distance is used because it is simple, deterministic, and aligns with the paper's reference formulation. It also makes it easy to reason about thresholds in normalized space.

Normalization is explicit so raw sensor units can vary independently of the matching metric. This keeps the protocol logic stable while allowing future calibration and sensor-specific mappings.

`epsilon` and `window_size` are parameters to keep the matching policy transparent and tunable. They allow experiments to trade off between sensitivity and stability without rewriting core logic.

## Out of Scope
This project does not implement real biosensor integrations, device firmware, or mobile apps. It focuses strictly on the protocol and its logic.
