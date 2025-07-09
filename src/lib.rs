#![allow(non_snake_case)]
pub mod breeding;
pub mod creature;
pub mod export;
pub mod library;
pub mod ocr;
pub mod overlay;
pub mod raising;
pub mod server_multipliers;
pub mod species;
pub mod stats;
pub mod taming;
pub mod utils;

use serde_json::Value;
use std::error::Error;

pub fn load_species() -> Result<Vec<species::Species>, Box<dyn Error>> {
    let data = std::fs::read_to_string("ARKBreedingStats/json/values/values.json")?;
    let json: Value = serde_json::from_str(&data)?;
    let species_value = json.get("species").ok_or("missing species")?.clone();
    let species: Vec<species::Species> = serde_json::from_value(species_value)?;
    Ok(species)
}

pub fn load_server_multipliers() -> Result<server_multipliers::ServerMultipliersFile, Box<dyn Error>>
{
    let data = std::fs::read_to_string("ARKBreedingStats/json/serverMultipliers.json")?;
    Ok(serde_json::from_str(&data)?)
}

pub fn load_server_multipliers_profile(
    profile: &str,
) -> Result<server_multipliers::ServerMultipliers, Box<dyn Error>> {
    let file = load_server_multipliers()?;
    file.server_multiplier_dictionary
        .get(profile)
        .cloned()
        .ok_or_else(|| format!("unknown profile: {profile}").into())
}
