use binary_dev_generator_plane::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 62, capacity: 85, latency: 12, risk: 18, weight: 12 };
    assert_eq!(score(signal), 95);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 76, capacity: 82, latency: 23, risk: 5, weight: 7 };
    assert_eq!(score(signal), 131);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 88, capacity: 85, latency: 12, risk: 11, weight: 10 };
    assert_eq!(score(signal), 178);
    assert_eq!(classify(signal), "accept");
}
