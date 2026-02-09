use phenomenological_rendezvous::srt::pattern_from_srt;
use phenomenological_rendezvous::{SemanticRendezvousToken, SubmodalityPattern};

fn assert_close(actual: f32, expected: f32, tol: f32, label: &str) {
    let diff = (actual - expected).abs();
    assert!(
        diff <= tol,
        "{label} out of tolerance: actual={actual} expected={expected} diff={diff}"
    );
}

fn assert_pattern_close(actual: &SubmodalityPattern, expected: &SubmodalityPattern, tol: f32) {
    assert_close(actual.brightness, expected.brightness, tol, "brightness");
    assert_close(actual.color_temp, expected.color_temp, tol, "color_temp");
    assert_close(actual.focal_distance, expected.focal_distance, tol, "focal_distance");
    assert_close(actual.volume, expected.volume, tol, "volume");
    assert_close(actual.tempo, expected.tempo, tol, "tempo");
    assert_close(actual.pitch, expected.pitch, tol, "pitch");
    assert_close(actual.temperature, expected.temperature, tol, "temperature");
    assert_close(actual.movement, expected.movement, tol, "movement");
    assert_close(actual.arousal, expected.arousal, tol, "arousal");
}

#[test]
fn srt_encoding_vector_alpha() {
    let srt = SemanticRendezvousToken::from_hex(
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    )
    .expect("valid hex");
    let actual = pattern_from_srt(&srt, b"alpha");
    let expected = SubmodalityPattern {
        brightness: 0.6505379,
        color_temp: 8464.454,
        focal_distance: 0.1207599,
        volume: 0.4094301,
        tempo: 119.63836,
        pitch: 15938.757,
        temperature: 25.549553,
        movement: 0.30618754,
        arousal: 0.6899062,
    };
    assert_pattern_close(&actual, &expected, 1e-3);
}

#[test]
fn srt_encoding_vector_beta() {
    let srt = SemanticRendezvousToken::from_hex(
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
    )
    .expect("valid hex");
    let actual = pattern_from_srt(&srt, b"beta");
    let expected = SubmodalityPattern {
        brightness: 0.043427177,
        color_temp: 4914.473,
        focal_distance: 0.5757839,
        volume: 0.5407492,
        tempo: 179.16228,
        pitch: 14068.652,
        temperature: 33.150837,
        movement: 0.7570611,
        arousal: 0.7669337,
    };
    assert_pattern_close(&actual, &expected, 1e-3);
}
