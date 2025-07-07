#![allow(dead_code)]
pub const STATS_COUNT: usize = 12;

pub const HEALTH: usize = 0;
/// Stamina or charge capacity
pub const STAMINA: usize = 1;
pub const TORPIDITY: usize = 2;
/// Oxygen or charge regeneration
pub const OXYGEN: usize = 3;
pub const FOOD: usize = 4;
pub const WATER: usize = 5;
pub const TEMPERATURE: usize = 6;
pub const WEIGHT: usize = 7;
/// Melee damage multiplier
pub const MELEE_DAMAGE_MULTIPLIER: usize = 8;
pub const SPEED_MULTIPLIER: usize = 9;
pub const TEMPERATURE_FORTITUDE: usize = 10;
pub const CRAFTING_SPEED_MULTIPLIER: usize = 11;

pub fn is_percentage(stat: usize) -> bool {
    matches!(
        stat,
        MELEE_DAMAGE_MULTIPLIER
            | SPEED_MULTIPLIER
            | TEMPERATURE_FORTITUDE
            | CRAFTING_SPEED_MULTIPLIER
    )
}

pub fn precision(stat: usize) -> usize {
    if is_percentage(stat) { 3 } else { 1 }
}
