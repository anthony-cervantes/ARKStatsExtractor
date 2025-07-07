use ARKStatsExtractor::{
    creature::{Creature, Sex},
    library::CreatureLibrary,
    stats::STATS_COUNT,
};
use tempfile::tempdir;

#[test]
fn save_and_load_library() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("lib.json");

    let mut lib = CreatureLibrary::default();
    let c = Creature::with_stats(
        "Bob".to_string(),
        "Rex".to_string(),
        Sex::Male,
        [1; STATS_COUNT],
        0,
    );
    lib.add(c.clone());
    lib.save(&file).unwrap();

    let lib2 = CreatureLibrary::load(&file).unwrap();
    assert_eq!(lib2.creatures.len(), 1);
    assert_eq!(lib2.creatures[0], c);
}

#[test]
fn remove_creature() {
    let mut lib = CreatureLibrary::default();
    lib.add(Creature::new(
        "Alice".to_string(),
        "Rex".to_string(),
        Sex::Female,
        0,
    ));
    assert!(lib.remove_by_name("Alice"));
    assert!(lib.creatures.is_empty());
}
