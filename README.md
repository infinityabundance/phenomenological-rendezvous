# Phenomenological Rendezvous

<p align="center">
  <a href="https://github.com/infinityabundance/phenomenological-rendezvous">
    <img src="/assets/phenomrndzv-dark2.svg" alt="" width="200"/>
  </a>
</p>

Phenomenological Rendezvous is a serverless peer coordination protocol that relies on matched internal representational patterns rather than centralized infrastructure. This repository provides a Rust reference implementation of the protocol-level building blocks.

The crate focuses on SRTs, pattern space definitions, matching logic, and simulation tooling. It does not include biosensor drivers, device firmware, or mobile applications. The paper is available at DOI: 10.5281/zenodo.18558066 (link: `https://doi.org/10.5281/zenodo.18558066`).

## Features
- SRT encoding primitives
- Submodality pattern data structures
- Matching and rendezvous utilities
- Simulation scaffolding for experiments

## Encoding
An SRT plus oracle-state (salt) deterministically maps to a SubmodalityPattern using HMAC-SHA256. The hash is partitioned into 16-bit segments and each segment is quantized into the appropriate range for its dimension. This yields a stable, reproducible pattern without exposing the secret.

## Status
Experimental reference implementation.

## Getting Started
```bash
cargo add phenomenological-rendezvous
```

```rust
use phenomenological_rendezvous::SemanticRendezvousToken;

let srt = SemanticRendezvousToken::from_bytes([0u8; 32]);
```

## License
Apache-2.0
