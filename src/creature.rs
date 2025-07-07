use serde::{Deserialize, Serialize};

use crate::{server_multipliers::ServerMultipliers, stats::STATS_COUNT};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Sex {
    Unknown,
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Creature {
    pub name: String,
    #[serde(default)]
    pub species: String,
    pub sex: Sex,
    pub stats: [i32; STATS_COUNT],
    #[serde(default)]
    pub mutations: i32,
}

impl Creature {
    pub fn new(name: String, species: String, sex: Sex, mutations: i32) -> Self {
        Self {
            name,
            species,
            sex,
            stats: [0; STATS_COUNT],
            mutations,
        }
    }

    pub fn with_stats(
        name: String,
        species: String,
        sex: Sex,
        stats: [i32; STATS_COUNT],
        mutations: i32,
    ) -> Self {
        Self {
            name,
            species,
            sex,
            stats,
            mutations,
        }
    }

    pub fn total_with_multipliers(&self, multipliers: Option<&ServerMultipliers>) -> f64 {
        self.stats
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let base = *s as f64;
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
            .sum()
    }
}
