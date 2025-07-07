use ARKStatsExtractor::{
    creature::{Creature, Sex},
    library::CreatureLibrary,
    load_species,
    stats::STATS_COUNT,
};
use clap::{Parser, Subcommand};

mod app;

#[derive(Subcommand)]
enum Commands {
    /// Add a creature to the library
    Add {
        name: String,
        #[arg(long)]
        sex: String,
    },
    /// Remove a creature from the library by name
    Remove { name: String },
}

#[derive(Parser)]
struct Args {
    /// Launch GUI instead of CLI
    #[arg(long)]
    gui: bool,

    /// Path to creature library JSON file
    #[arg(long, default_value = "creatures.json")]
    library: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn run_cli(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut lib = CreatureLibrary::load(&args.library)?;
    match &args.command {
        Some(Commands::Add { name, sex }) => {
            let sex = match sex.to_lowercase().as_str() {
                "male" | "m" => Sex::Male,
                "female" | "f" => Sex::Female,
                _ => Sex::Unknown,
            };
            let creature = Creature::with_stats(name.clone(), sex, [0; STATS_COUNT], 0);
            lib.add(creature);
            lib.save(&args.library)?;
        }
        Some(Commands::Remove { name }) => {
            if lib.remove_by_name(name) {
                lib.save(&args.library)?;
            } else {
                eprintln!("Creature not found: {name}");
            }
        }
        None => {
            for c in &lib.creatures {
                println!("{} ({:?})", c.name, c.sex);
            }
        }
    }
    Ok(())
}

fn main() -> eframe::Result<()> {
    let args = Args::parse();
    if args.gui {
        let species = load_species().expect("load species");
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "ARK Stats",
            options,
            Box::new(|_cc| Ok(Box::new(app::SpeciesApp::new(species)))),
        )?;
    } else {
        run_cli(&args).expect("run cli");
    }
    Ok(())
}
