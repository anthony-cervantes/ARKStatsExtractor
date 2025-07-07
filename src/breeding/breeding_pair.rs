use crate::{breeding::score::Score, creature::Creature};

#[derive(Debug, Clone)]
pub struct BreedingPair {
    pub mother: Creature,
    pub father: Creature,
    pub breeding_score: Score,
    /// Probability of at least one mutation for the offspring.
    pub mutation_probability: f64,
    /// The highest possible offspring is over the level limit if all possible dom levels are applied.
    pub highest_offspring_over_level_limit: bool,
}

impl BreedingPair {
    pub fn new(
        mother: Creature,
        father: Creature,
        breeding_score: Score,
        mutation_probability: f64,
        highest_offspring_over_level_limit: bool,
    ) -> Self {
        Self {
            mother,
            father,
            breeding_score,
            mutation_probability,
            highest_offspring_over_level_limit,
        }
    }

    pub fn from_parents(mother: Creature, father: Creature) -> Self {
        let breeding_score = calculate_score(&mother, &father);
        Self::new(mother, father, breeding_score, 0.0, false)
    }
}

pub fn calculate_score(mother: &Creature, father: &Creature) -> Score {
    let total: i32 = mother
        .stats
        .iter()
        .zip(father.stats.iter())
        .map(|(m, f)| (*m).max(*f))
        .sum();
    Score::primary(total as f64)
}
