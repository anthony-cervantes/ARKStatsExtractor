use ARKStatsExtractor::{
    breeding::breeding_pair::calculate_score,
    creature::{Creature, Sex},
    load_server_multipliers_profile,
    stats::STATS_COUNT,
};

#[test]
fn creature_total_with_multipliers() {
    let c = Creature::with_stats(
        "Test".to_string(),
        "Rex".to_string(),
        Sex::Male,
        [10; STATS_COUNT],
        0,
    );
    let sm = load_server_multipliers_profile("official").unwrap();
    let total = c.total_with_multipliers(Some(&sm));
    let expected = calculate_score(&c, &c, Some(&sm)).one_number();
    assert!((total - expected).abs() < f64::EPSILON);
}
