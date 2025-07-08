use std::fs;
use std::path::Path;

use crate::creature::Creature;

/// Export a creature to a JSON file.
pub fn export_creature<P: AsRef<Path>>(
    creature: &Creature,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string_pretty(creature)?;
    fs::write(path, data)?;
    Ok(())
}

/// Import a creature from a JSON file.
pub fn import_creature<P: AsRef<Path>>(path: P) -> Result<Creature, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creature::{Creature, Sex};
    use crate::stats::STATS_COUNT;
    use tempfile::NamedTempFile;

    #[test]
    fn export_and_import_creature() {
        let creature = Creature::with_stats(
            "Bob".to_string(),
            "Rex".to_string(),
            Sex::Male,
            [1; STATS_COUNT],
            0,
        );
        let file = NamedTempFile::new().unwrap();
        export_creature(&creature, file.path()).unwrap();
        let loaded = import_creature(file.path()).unwrap();
        assert_eq!(creature, loaded);
    }
}
