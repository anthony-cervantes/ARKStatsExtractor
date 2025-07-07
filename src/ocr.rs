use crate::{
    creature::{Creature, Sex},
    stats::STATS_COUNT,
};
use leptess::LepTess;
use std::path::Path;

/// Read creature data from an image using OCR.
pub fn read_creature_from_image<P: AsRef<Path>>(
    path: P,
) -> Result<Creature, Box<dyn std::error::Error>> {
    let mut lt = LepTess::new(None, "eng")?;
    lt.set_image(path.as_ref())?;
    let text = lt.get_utf8_text()?;
    parse_creature(&text)
}

fn parse_creature(text: &str) -> Result<Creature, Box<dyn std::error::Error>> {
    let mut name = String::new();
    let mut species = String::new();
    let mut sex = Sex::Unknown;
    for line in text.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Name:") {
            name = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("Species:") {
            species = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("Sex:") {
            let s = rest.trim().to_lowercase();
            sex = match s.as_str() {
                "male" | "m" => Sex::Male,
                "female" | "f" => Sex::Female,
                _ => Sex::Unknown,
            };
        }
    }
    if name.is_empty() || species.is_empty() {
        return Err("failed to parse creature".into());
    }
    Ok(Creature::with_stats(
        name,
        species,
        sex,
        [0; STATS_COUNT],
        0,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_creature_basic() {
        let text = "Name: Bob\nSpecies: Rex\nSex: Male";
        let c = parse_creature(text).unwrap();
        assert_eq!(c.name, "Bob");
        assert_eq!(c.species, "Rex");
        assert_eq!(c.sex, Sex::Male);
    }
}
