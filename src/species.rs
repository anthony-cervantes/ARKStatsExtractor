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
}
