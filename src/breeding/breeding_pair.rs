use crate::{
    breeding::score::Score, creature::Creature, server_multipliers::ServerMultipliers,
    stats::STATS_COUNT,
};

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
        let mutation_probability = calculate_mutation_chance(&mother, &father);
        Self::new(mother, father, breeding_score, mutation_probability, false)
    }

    pub fn offspring_levels(&self) -> [i32; STATS_COUNT] {
        possible_offspring_levels(&self.mother, &self.father)
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

pub fn possible_offspring_levels(mother: &Creature, father: &Creature) -> [i32; STATS_COUNT] {
    let mut levels = [0; STATS_COUNT];
    for (i, level) in levels.iter_mut().enumerate().take(STATS_COUNT) {
        *level = mother.stats[i].max(father.stats[i]);
    }
    levels
}

pub fn calculate_mutation_chance(mother: &Creature, father: &Creature) -> f64 {
    let m = if mother.mutations < 20 { 0.025 } else { 0.0 };
    let f = if father.mutations < 20 { 0.025 } else { 0.0 };
    1.0 - (1.0 - m) * (1.0 - f)
}
