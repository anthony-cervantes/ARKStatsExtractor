use ARKStatsExtractor::breeding::score::Score;

#[test]
fn score_ordering() {
    let a = Score::new(2.0, 0.0, 0.0);
    let b = Score::new(1.0, 10.0, 0.0);
    assert!(a > b);
}

#[test]
fn score_equality() {
    let a = Score::new(1.0, 2.0, 3.0);
    let b = Score::new(1.0, 2.0, 3.0);
    assert_eq!(a, b);
}

#[test]
fn score_one_number() {
    let s = Score::new(1.0, 2.0, 3.0);
    let n = s.one_number();
    assert!((n - 1.0203).abs() < f64::EPSILON);
}
