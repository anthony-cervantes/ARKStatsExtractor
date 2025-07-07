use ARKStatsExtractor::{load_species, species::Species};
use clap::Parser;

mod app;

#[derive(Parser)]
struct Args {
    /// Launch GUI instead of CLI
    #[arg(long)]
    gui: bool,
}

fn run_cli(species: &[Species]) {
    for s in species {
        println!("{}", s.name);
    }
}

fn main() -> eframe::Result<()> {
    let args = Args::parse();
    let species = load_species().expect("load species");
    if args.gui {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "ARK Stats",
            options,
            Box::new(|_cc| Ok(Box::new(app::SpeciesApp::new(species)))),
        )?;
    } else {
        run_cli(&species);
    }
    Ok(())
}
