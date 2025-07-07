use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ServerMultipliers {
    #[serde(rename = "statMultipliers", default)]
    pub stat_multipliers: Option<Vec<Option<[f64; 4]>>>,
    #[serde(rename = "TamingSpeedMultiplier", default)]
    pub taming_speed_multiplier: Option<f64>,
    #[serde(rename = "WildDinoTorporDrainMultiplier", default)]
    pub wild_dino_torpor_drain_multiplier: Option<f64>,
    #[serde(rename = "DinoCharacterFoodDrainMultiplier", default)]
    pub dino_character_food_drain_multiplier: Option<f64>,
    #[serde(rename = "TamedDinoCharacterFoodDrainMultiplier", default)]
    pub tamed_dino_character_food_drain_multiplier: Option<f64>,
    #[serde(rename = "WildDinoCharacterFoodDrainMultiplier", default)]
    pub wild_dino_character_food_drain_multiplier: Option<f64>,
    #[serde(rename = "MatingSpeedMultiplier", default)]
    pub mating_speed_multiplier: Option<f64>,
    #[serde(rename = "MatingIntervalMultiplier", default)]
    pub mating_interval_multiplier: Option<f64>,
    #[serde(rename = "EggHatchSpeedMultiplier", default)]
    pub egg_hatch_speed_multiplier: Option<f64>,
    #[serde(rename = "BabyMatureSpeedMultiplier", default)]
    pub baby_mature_speed_multiplier: Option<f64>,
    #[serde(rename = "BabyFoodConsumptionSpeedMultiplier", default)]
    pub baby_food_consumption_speed_multiplier: Option<f64>,
    #[serde(rename = "BabyCuddleIntervalMultiplier", default)]
    pub baby_cuddle_interval_multiplier: Option<f64>,
    #[serde(rename = "BabyImprintingStatScaleMultiplier", default)]
    pub baby_imprinting_stat_scale_multiplier: Option<f64>,
    #[serde(rename = "BabyImprintAmountMultiplier", default)]
    pub baby_imprint_amount_multiplier: Option<f64>,
    #[serde(rename = "AllowSpeedLeveling", default)]
    pub allow_speed_leveling: Option<bool>,
    #[serde(rename = "AllowFlyerSpeedLeveling", default)]
    pub allow_flyer_speed_leveling: Option<bool>,
    #[serde(rename = "SinglePlayerSettings", default)]
    pub single_player_settings: Option<bool>,
    #[serde(rename = "AtlasSettings", default)]
    pub atlas_settings: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServerMultipliersFile {
    pub format: String,
    pub version: String,
    #[serde(rename = "serverMultiplierDictionary", default)]
    pub server_multiplier_dictionary: std::collections::HashMap<String, ServerMultipliers>,
}
