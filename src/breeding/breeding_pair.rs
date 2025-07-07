use crate::{breeding::score::Score, creature::Creature, server_multipliers::ServerMultipliers};

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

    pub fn from_parents(
        mother: Creature,
        father: Creature,
        multipliers: Option<&ServerMultipliers>,
    ) -> Self {
        let breeding_score = calculate_score(&mother, &father, multipliers);
        Self::new(mother, father, breeding_score, 0.0, false)
    }
}

pub fn calculate_score(
    mother: &Creature,
    father: &Creature,
    multipliers: Option<&ServerMultipliers>,
) -> Score {
    let total: f64 = mother
        .stats
        .iter()
        .zip(father.stats.iter())
        .enumerate()
        .map(|(i, (m, f))| {
            let base = (*m).max(*f) as f64;
            if let Some(sm) = multipliers {
                if let Some(stat_mults) = sm
                    .stat_multipliers
                    .as_ref()
                    .and_then(|v| v.get(i))
                    .and_then(|o| o.as_ref())
                {
                    base * stat_mults[ServerMultipliers::INDEX_LEVEL_DOM]
                } else {
                    base
                }
            } else {
                base
            }
        })
        .sum();
    Score::primary(total)
}
