use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::creature::Creature;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CreatureLibrary {
    pub creatures: Vec<Creature>,
}

impl CreatureLibrary {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        if !path.as_ref().exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn add(&mut self, creature: Creature) {
        self.creatures.push(creature);
    }

    pub fn remove_by_name(&mut self, name: &str) -> bool {
        if let Some(pos) = self.creatures.iter().position(|c| c.name == name) {
            self.creatures.remove(pos);
            true
        } else {
            false
        }
    }
}
