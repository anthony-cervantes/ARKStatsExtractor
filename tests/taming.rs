use ARKStatsExtractor::{
    server_multipliers::ServerMultipliers,
    species::{Species, TamingData},
    taming::get_taming_info,
};

#[test]
fn taming_info_basic() {
    let species = Species {
        name: "Test".into(),
        blueprint_path: "path".into(),
        variants: vec![],
        full_stats_raw: vec![],
        breeding: None,
        taming: Some(TamingData {
            non_violent: false,
            violent: true,
            taming_ineffectiveness: 1.0,
            affinity_needed0: 10.0,
            affinity_increase_pl: 2.0,
            food_consumption_base: 1.0,
            food_consumption_mult: 2.0,
            baby_food_consumption_mult: None,
        }),
    };
    let mult = ServerMultipliers {
        taming_speed_multiplier: Some(2.0),
        ..Default::default()
    };
    let info = get_taming_info(&species, Some(&mult)).unwrap();
    assert!((info.total_time.as_secs_f64() - 5.0).abs() < f64::EPSILON);
    assert!((info.food_consumed - 5.0).abs() < f64::EPSILON);
}
