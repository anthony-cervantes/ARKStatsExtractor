use serde::{Deserialize, Serialize};

use crate::stats::STATS_COUNT;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Sex {
    Unknown,
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creature {
    pub name: String,
    pub sex: Sex,
    pub stats: [i32; STATS_COUNT],
    #[serde(default)]
    pub mutations: i32,
}

impl Creature {
    pub fn new(name: String, sex: Sex, mutations: i32) -> Self {
        Self {
            name,
            sex,
            stats: [0; STATS_COUNT],
            mutations,
        }
    }

    pub fn with_stats(name: String, sex: Sex, stats: [i32; STATS_COUNT], mutations: i32) -> Self {
        Self {
            name,
            sex,
            stats,
            mutations,
        }
    }
}
