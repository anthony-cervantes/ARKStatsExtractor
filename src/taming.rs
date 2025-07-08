use std::time::Duration;

use crate::{
    server_multipliers::ServerMultipliers,
    species::{Species, TamingData},
};

#[derive(Debug)]
pub struct TamingInfo {
    pub total_time: Duration,
    pub food_consumed: f64,
}

pub fn get_taming_info(
    species: &Species,
    multipliers: Option<&ServerMultipliers>,
) -> Option<TamingInfo> {
    let t: &TamingData = species.taming.as_ref()?;
    let taming_speed = multipliers
        .and_then(|m| m.taming_speed_multiplier)
        .unwrap_or(1.0);

    if t.affinity_increase_pl <= 0.0 {
        return None;
    }

    let food_count = (t.affinity_needed0 / t.affinity_increase_pl).ceil();
    let time_per_food = t.food_consumption_base * t.food_consumption_mult;
    let total_time = Duration::from_secs_f64(food_count * time_per_food / taming_speed);

    Some(TamingInfo {
        total_time,
        food_consumed: food_count,
    })
}
