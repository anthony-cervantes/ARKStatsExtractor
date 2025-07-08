use std::time::Duration;

use crate::{
    server_multipliers::ServerMultipliers,
    species::{BreedingData, Species},
};

#[derive(Debug)]
pub struct RaisingTimes {
    pub incubation_mode: String,
    pub incubation: Duration,
    pub baby: Duration,
    pub maturation: Duration,
    pub next_mating_min: Duration,
    pub next_mating_max: Duration,
}

pub fn get_raising_times(
    species: &Species,
    multipliers: Option<&ServerMultipliers>,
) -> Option<RaisingTimes> {
    let breeding: &BreedingData = species.breeding.as_ref()?;
    let egg_mult = multipliers
        .and_then(|m| m.egg_hatch_speed_multiplier)
        .unwrap_or(1.0);
    let baby_mult = multipliers
        .and_then(|m| m.baby_mature_speed_multiplier)
        .unwrap_or(1.0);
    let interval_mult = multipliers
        .and_then(|m| m.mating_interval_multiplier)
        .unwrap_or(1.0);

    let gestation = breeding.gestation_time / egg_mult;
    let incubation = breeding.incubation_time / egg_mult;
    let mode = if gestation == 0.0 {
        "Incubation"
    } else {
        "Gestation"
    };
    let baby_total = breeding.maturation_time / baby_mult * 0.1;
    let maturation_total = breeding.maturation_time / baby_mult;
    let next_min = breeding.mating_cooldown_min * interval_mult;
    let next_max = breeding.mating_cooldown_max * interval_mult;

    Some(RaisingTimes {
        incubation_mode: mode.to_string(),
        incubation: Duration::from_secs_f64(gestation + incubation),
        baby: Duration::from_secs_f64(baby_total),
        maturation: Duration::from_secs_f64(maturation_total),
        next_mating_min: Duration::from_secs_f64(next_min),
        next_mating_max: Duration::from_secs_f64(next_max),
    })
}
