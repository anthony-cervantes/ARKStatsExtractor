use ARKStatsExtractor::{
    load_server_multipliers_profile, server_multipliers::ServerMultipliersFile, species::Species,
};

#[test]
fn load_species_json() {
    let data = std::fs::read_to_string("ARKBreedingStats/json/values/values.json")
        .expect("read values.json");
    let json: serde_json::Value = serde_json::from_str(&data).expect("parse json");
    let species_value = json.get("species").expect("species array").clone();
    let species: Vec<Species> = serde_json::from_value(species_value).expect("deserialize species");
    assert!(!species.is_empty());
}

#[test]
fn load_server_multipliers() {
    let data = std::fs::read_to_string("ARKBreedingStats/json/serverMultipliers.json")
        .expect("read serverMultipliers.json");
    let sm: ServerMultipliersFile =
        serde_json::from_str(&data).expect("deserialize server multipliers");
    assert!(sm.server_multiplier_dictionary.contains_key("official"));
}

#[test]
fn load_server_multipliers_profile_function() {
    let sm = load_server_multipliers_profile("singleplayer").unwrap();
    assert!(sm.mating_interval_multiplier.is_some());
}
