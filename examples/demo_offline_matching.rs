use phenomenological_rendezvous::matching::{MatchingConfig, Matcher};
use phenomenological_rendezvous::srt::pattern_from_srt;
use phenomenological_rendezvous::{SemanticRendezvousToken, SubmodalityPattern};

fn main() {
    let srt = SemanticRendezvousToken::from_hex(
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    )
    .expect("valid hex");
    let target = pattern_from_srt(&srt, b"oracle-state");

    let measured_stream = vec![
        SubmodalityPattern::zeros(),
        SubmodalityPattern {
            brightness: 0.52,
            volume: 0.47,
            ..SubmodalityPattern::zeros()
        },
        SubmodalityPattern {
            brightness: 0.51,
            volume: 0.48,
            ..SubmodalityPattern::zeros()
        },
        SubmodalityPattern {
            brightness: 0.5,
            volume: 0.49,
            ..SubmodalityPattern::zeros()
        },
    ];

    let mut matcher = Matcher::new(MatchingConfig::new(0.2, 2));

    for (index, measured) in measured_stream.iter().enumerate() {
        let matched = matcher.observe(measured, &target);
        if matched {
            println!("rendezvous triggered at index {index}");
        } else {
            println!("no match at index {index}");
        }
    }
}
