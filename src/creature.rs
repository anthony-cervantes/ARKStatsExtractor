use serde::{Deserialize, Serialize};

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
    #[serde(default)]
    pub mutations: i32,
}

impl Creature {
    pub fn new(name: String, sex: Sex, mutations: i32) -> Self {
        Self {
            name,
            sex,
            mutations,
        }
    }
}
