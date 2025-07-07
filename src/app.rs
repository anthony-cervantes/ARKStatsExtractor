use eframe::egui;
use std::path::PathBuf;

use ARKStatsExtractor::{
    creature::{Creature, Sex},
    library::CreatureLibrary,
    species::Species,
};

pub struct LibraryApp {
    library: CreatureLibrary,
    species: Vec<Species>,
    library_path: PathBuf,
    filter_species: Option<String>,
    filter_sex: Option<Sex>,
    new_name: String,
    new_species: String,
    new_sex: Sex,
}

impl LibraryApp {
    pub fn new(species: Vec<Species>, library: CreatureLibrary, library_path: PathBuf) -> Self {
        Self {
            library,
            species,
            library_path,
            filter_species: None,
            filter_sex: None,
            new_name: String::new(),
            new_species: String::new(),
            new_sex: Sex::Unknown,
        }
    }

    fn save_to_file(&self) {
        let _ = self.library.save(&self.library_path);
    }
}

impl eframe::App for LibraryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Creature Library");

            ui.horizontal(|ui| {
                ui.label("Filter species:");
                egui::ComboBox::from_id_salt("species_filter")
                    .selected_text(self.filter_species.clone().unwrap_or_else(|| "All".into()))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.filter_species, None, "All");
                        for s in &self.species {
                            ui.selectable_value(
                                &mut self.filter_species,
                                Some(s.name.clone()),
                                &s.name,
                            );
                        }
                    });

                ui.label("Filter sex:");
                egui::ComboBox::from_id_salt("sex_filter")
                    .selected_text(match self.filter_sex {
                        Some(Sex::Male) => "Male",
                        Some(Sex::Female) => "Female",
                        _ => "Any",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.filter_sex, None, "Any");
                        ui.selectable_value(&mut self.filter_sex, Some(Sex::Male), "Male");
                        ui.selectable_value(&mut self.filter_sex, Some(Sex::Female), "Female");
                    });
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.new_name);
                ui.label("Species:");
                egui::ComboBox::from_id_salt("new_species")
                    .selected_text(if self.new_species.is_empty() {
                        "Select".into()
                    } else {
                        self.new_species.clone()
                    })
                    .show_ui(ui, |ui| {
                        for s in &self.species {
                            ui.selectable_value(&mut self.new_species, s.name.clone(), &s.name);
                        }
                    });
                ui.label("Sex:");
                egui::ComboBox::from_id_salt("new_sex")
                    .selected_text(match self.new_sex {
                        Sex::Male => "Male",
                        Sex::Female => "Female",
                        Sex::Unknown => "Unknown",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.new_sex, Sex::Unknown, "Unknown");
                        ui.selectable_value(&mut self.new_sex, Sex::Male, "Male");
                        ui.selectable_value(&mut self.new_sex, Sex::Female, "Female");
                    });

                if ui.button("Add").clicked()
                    && !self.new_name.is_empty()
                    && !self.new_species.is_empty()
                {
                    let creature = Creature::new(
                        self.new_name.clone(),
                        self.new_species.clone(),
                        self.new_sex.clone(),
                        0,
                    );
                    self.library.add(creature);
                    self.save_to_file();
                    self.new_name.clear();
                }
            });

            ui.separator();

            let mut remove_index: Option<usize> = None;
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, c) in self.library.creatures.iter().enumerate() {
                    if self
                        .filter_species
                        .as_ref()
                        .is_some_and(|f| &c.species != f)
                    {
                        continue;
                    }
                    if self.filter_sex.as_ref().is_some_and(|s| &c.sex != s) {
                        continue;
                    }
                    ui.horizontal(|ui| {
                        ui.label(format!("{} - {} ({:?})", c.name, c.species, c.sex));
                        if ui.button("Remove").clicked() {
                            remove_index = Some(idx);
                        }
                    });
                }
            });
            if let Some(idx) = remove_index {
                self.library.creatures.remove(idx);
                self.save_to_file();
            }
        });
    }
}
