use phenomenological_rendezvous::sim::{run_simulation, SimulationConfig};
use phenomenological_rendezvous::SemanticRendezvousToken;

fn main() {
    let config = SimulationConfig {
        num_peers: 500,
        num_trials: 200,
        epsilon: 0.15,
        window_size: 1,
        apply_geo_filter: true,
        geo_filter_factor: 1e6,
    };

    let srt = SemanticRendezvousToken::from_bytes([1u8; 32]);
    let result = run_simulation(&config, &srt, b"oracle-state");

    println!("Simulation summary:\n{result:#?}");
}
