use ARKStatsExtractor::{
    raising::get_raising_times,
    server_multipliers::ServerMultipliers,
    species::{BreedingData, Species},
};

#[test]
fn raising_times_basic() {
    let species = Species {
        name: "Test".into(),
        blueprint_path: "path".into(),
        variants: vec![],
        full_stats_raw: vec![],
        taming: None,
        breeding: Some(BreedingData {
            gestation_time: 100.0,
            incubation_time: 200.0,
            maturation_time: 1000.0,
            mating_cooldown_min: 600.0,
            mating_cooldown_max: 1200.0,
            egg_temp_min: None,
            egg_temp_max: None,
        }),
    };
    let mult = ServerMultipliers {
        egg_hatch_speed_multiplier: Some(2.0),
        baby_mature_speed_multiplier: Some(4.0),
        mating_interval_multiplier: Some(0.5),
        ..Default::default()
    };
    let times = get_raising_times(&species, Some(&mult)).unwrap();
    assert!((times.incubation.as_secs_f64() - 150.0).abs() < f64::EPSILON);
    assert!((times.baby.as_secs_f64() - 25.0).abs() < f64::EPSILON);
    assert!((times.maturation.as_secs_f64() - 250.0).abs() < f64::EPSILON);
    assert!((times.next_mating_min.as_secs_f64() - 300.0).abs() < f64::EPSILON);
    assert!((times.next_mating_max.as_secs_f64() - 600.0).abs() < f64::EPSILON);
    assert_eq!(times.incubation_mode, "Gestation");
}
