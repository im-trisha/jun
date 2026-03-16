use egui::{Key, Modifiers};

use crate::JunApp;

impl JunApp {
    fn background(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                ui.add(
                    egui::Image::new(egui::include_image!(
                        "../../../assets/characters/Annalie.png"
                    ))
                    .max_height(500.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                    ui.add(
                        egui::Image::new(egui::include_image!(
                            "../../../assets/characters/Shanice.png"
                        ))
                        .max_height(500.0),
                    );
                });
            });
        });
    }

    pub(super) fn content(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.background(ctx);

        let content_frame = egui::containers::Frame::default();
        egui::CentralPanel::default()
            .frame(content_frame)
            .show(ctx, |ui| {
                if ui.input_mut(|i| i.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::S))
                    && let Some(path) = self.mdrg_file_dialog().save_file()
                {
                    self.export_save(path);
                }

                if ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::S))
                    && let Some(path) = self.state.worked_with.last()
                {
                    self.export_save(path.clone());
                }

                self.current_screen.show(ui, &mut self.state)
            });
    }
}
