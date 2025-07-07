use ARKStatsExtractor::{
    breeding::{
        breeding_pair::{BreedingPair, calculate_score},
        score::Score,
    },
    creature::{Creature, Sex},
    stats::STATS_COUNT,
};

#[test]
fn breeding_pair_creation() {
    let mother = Creature::new("Mom".to_string(), Sex::Female, 0);
    let father = Creature::new("Dad".to_string(), Sex::Male, 0);
    let score = Score::primary(2.0);
    let bp = BreedingPair::new(mother.clone(), father.clone(), score, 0.25, false);
    assert_eq!(bp.mother.name, "Mom");
    assert_eq!(bp.father.name, "Dad");
    assert!((bp.mutation_probability - 0.25).abs() < f64::EPSILON);
    assert!(!bp.highest_offspring_over_level_limit);
    assert_eq!(bp.breeding_score.one_number(), 2.0);
}

#[test]
fn breeding_pair_score_calculation() {
    let mother = Creature::with_stats("Mom".to_string(), Sex::Female, [10; STATS_COUNT], 0);
    let father = Creature::with_stats("Dad".to_string(), Sex::Male, [5; STATS_COUNT], 0);
    let score = calculate_score(&mother, &father);
    assert_eq!(score, Score::primary((10 * STATS_COUNT as i32) as f64));
}
