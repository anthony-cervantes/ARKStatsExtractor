use ARKStatsExtractor::{
    breeding::{breeding_pair::BreedingPair, score::Score},
    creature::{Creature, Sex},
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
