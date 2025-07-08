use eframe::egui;

pub struct OverlayApp {
    message: String,
}

impl OverlayApp {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl eframe::App for OverlayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.message);
        });
    }
}
