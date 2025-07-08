use ARKStatsExtractor::{
    breeding::{
        breeding_pair::{
            BreedingPair, calculate_mutation_chance, calculate_score, expected_offspring_levels,
            expected_offspring_score, possible_offspring_levels,
        },
        score::Score,
    },
    creature::{Creature, Sex},
    load_server_multipliers_profile,
    stats::STATS_COUNT,
};

#[test]
fn breeding_pair_creation() {
    let mother = Creature::new("Mom".to_string(), "Rex".to_string(), Sex::Female, 0);
    let father = Creature::new("Dad".to_string(), "Rex".to_string(), Sex::Male, 0);
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
    let mother = Creature::with_stats(
        "Mom".to_string(),
        "Rex".to_string(),
        Sex::Female,
        [10; STATS_COUNT],
        0,
    );
    let father = Creature::with_stats(
        "Dad".to_string(),
        "Rex".to_string(),
        Sex::Male,
        [5; STATS_COUNT],
        0,
    );
    let sm = load_server_multipliers_profile("official").unwrap();
    let score = calculate_score(&mother, &father, Some(&sm));
    assert!((score.one_number() - 103.7).abs() < f64::EPSILON);
}

#[test]
fn offspring_levels_and_mutation_chance() {
    let mother = Creature::with_stats(
        "Mom".to_string(),
        "Rex".to_string(),
        Sex::Female,
        [10; STATS_COUNT],
        0,
    );
    let father = Creature::with_stats(
        "Dad".to_string(),
        "Rex".to_string(),
        Sex::Male,
        [8; STATS_COUNT],
        10,
    );
    let pair = BreedingPair::from_parents(mother.clone(), father.clone(), None);
    let expected_levels = possible_offspring_levels(&mother, &father);
    assert_eq!(pair.offspring_levels(), expected_levels);

    let expected = calculate_mutation_chance(&mother, &father);
    assert!((pair.mutation_probability - expected).abs() < f64::EPSILON);
}

#[test]
fn expected_offspring_functions() {
    let mother = Creature::with_stats(
        "Mom".to_string(),
        "Rex".to_string(),
        Sex::Female,
        [10; STATS_COUNT],
        0,
    );
    let father = Creature::with_stats(
        "Dad".to_string(),
        "Rex".to_string(),
        Sex::Male,
        [0; STATS_COUNT],
        0,
    );
    let expected_levels = [7.0; STATS_COUNT];
    assert_eq!(expected_offspring_levels(&mother, &father), expected_levels);
    let score = expected_offspring_score(&mother, &father, None);
    assert!((score.one_number() - 7.0 * STATS_COUNT as f64).abs() < f64::EPSILON);
}
