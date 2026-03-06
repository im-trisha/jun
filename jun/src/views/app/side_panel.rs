use crate::JunApp;

impl JunApp {
    pub(super) fn side_panel(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("window_options_side_panel").show(ctx, |ui| {
            ui.label("Hello World!");
        });
    }
}
