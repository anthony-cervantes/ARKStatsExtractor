use ARKStatsExtractor::species::Species;
use eframe::egui;

pub struct SpeciesApp {
    species: Vec<Species>,
}

impl SpeciesApp {
    pub fn new(species: Vec<Species>) -> Self {
        Self { species }
    }
}

impl eframe::App for SpeciesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Species");
            for s in &self.species {
                ui.label(&s.name);
            }
        });
    }
}
