use binary_dev_generator_plane::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 46, slack: 31, drag: 10, confidence: 89 };
    assert_eq!(review_score(case), 182);
    assert_eq!(review_lane(case), "ship");
}
