use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Species {
    pub name: String,
    #[serde(rename = "blueprintPath")]
    pub blueprint_path: String,
    #[serde(default)]
    pub variants: Vec<String>,
    #[serde(rename = "fullStatsRaw", default)]
    pub full_stats_raw: Vec<Option<[f64; 5]>>,
    #[serde(default)]
    pub breeding: Option<BreedingData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BreedingData {
    #[serde(rename = "gestationTime", default)]
    pub gestation_time: f64,
    #[serde(rename = "incubationTime", default)]
    pub incubation_time: f64,
    #[serde(rename = "maturationTime", default)]
    pub maturation_time: f64,
    #[serde(rename = "matingCooldownMin", default)]
    pub mating_cooldown_min: f64,
    #[serde(rename = "matingCooldownMax", default)]
    pub mating_cooldown_max: f64,
    #[serde(rename = "eggTempMin", default)]
    pub egg_temp_min: Option<f64>,
    #[serde(rename = "eggTempMax", default)]
    pub egg_temp_max: Option<f64>,
}
