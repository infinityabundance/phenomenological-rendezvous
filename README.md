# Phenomenological Rendezvous

Phenomenological Rendezvous is a serverless peer coordination protocol that relies on matched internal representational patterns rather than centralized infrastructure. This repository provides a Rust reference implementation of the protocol-level building blocks.

The crate focuses on SRTs, pattern space definitions, matching logic, and simulation tooling. It does not include biosensor drivers, device firmware, or mobile applications. The paper is available at DOI: 10.5281/zenodo.18558066 (link: `https://doi.org/10.5281/zenodo.18558066`).

## Features
- SRT encoding primitives
- Submodality pattern data structures
- Matching and rendezvous utilities
- Simulation scaffolding for experiments

## Status
Experimental reference implementation.

## Getting Started
```bash
cargo add phenomenological-rendezvous
```

```rust
use phenomenological_rendezvous::srt::Srt;

let srt = Srt::new();
```

## License
Apache-2.0
