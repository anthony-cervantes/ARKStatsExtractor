#![allow(non_snake_case)]
pub mod server_multipliers;
pub mod species;
pub mod stats;

use serde_json::Value;
use std::error::Error;

pub fn load_species() -> Result<Vec<species::Species>, Box<dyn Error>> {
    let data = std::fs::read_to_string("ARKBreedingStats/json/values/values.json")?;
    let json: Value = serde_json::from_str(&data)?;
    let species_value = json.get("species").ok_or("missing species")?.clone();
    let species: Vec<species::Species> = serde_json::from_value(species_value)?;
    Ok(species)
}
