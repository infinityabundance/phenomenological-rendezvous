use phenomenological_rendezvous::matching::{MatchingConfig, Matcher};
use phenomenological_rendezvous::pattern::{
    SubmodalityPattern, AROUSAL_MAX, AROUSAL_MIN, BRIGHTNESS_MAX, BRIGHTNESS_MIN,
};

#[test]
fn matching_respects_window_size() {
    let mut matcher = Matcher::new(MatchingConfig::new(0.05, 3));
    let measured = SubmodalityPattern::zeros();
    let target = SubmodalityPattern::zeros();

    assert!(!matcher.observe(&measured, &target));
    assert!(!matcher.observe(&measured, &target));
    assert!(matcher.observe(&measured, &target));
}

#[test]
fn matching_rejects_far_patterns() {
    let mut matcher = Matcher::new(MatchingConfig::new(0.1, 2));
    let measured = SubmodalityPattern {
        brightness: BRIGHTNESS_MIN,
        arousal: AROUSAL_MIN,
        ..SubmodalityPattern::zeros()
    };
    let target = SubmodalityPattern {
        brightness: BRIGHTNESS_MAX,
        arousal: AROUSAL_MAX,
        ..SubmodalityPattern::zeros()
    };

    assert!(!matcher.observe(&measured, &target));
    assert!(!matcher.observe(&measured, &target));
}

#[test]
fn epsilon_changes_matching_behavior() {
    let measured = SubmodalityPattern::zeros();
    let target = SubmodalityPattern {
        brightness: BRIGHTNESS_MAX,
        ..SubmodalityPattern::zeros()
    };

    let mut strict = Matcher::new(MatchingConfig::new(0.01, 1));
    let mut loose = Matcher::new(MatchingConfig::new(1.5, 1));

    assert!(!strict.observe(&measured, &target));
    assert!(loose.observe(&measured, &target));
}
