use egui::{Frame, Key, Modifiers};

use crate::JunApp;

impl JunApp {
    fn background(ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                ui.set_opacity(0.6);
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
        let central_panel = if self.state.freaky {
            Self::background(ctx);
            egui::CentralPanel::default().frame(Frame::new().inner_margin(8))
        } else {
            egui::CentralPanel::default()
        };

        central_panel.show(ctx, |ui| {
            if ui.input_mut(|i| i.consume_key(Modifiers::CTRL | Modifiers::SHIFT, Key::S))
                && let Some(path) = self.mdrg_file_dialog().save_file()
            {
                self.export_save(path);
            }

            if ui.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::S))
                && let Some(path) = self.state.recent_paths.last()
            {
                self.export_save(path.clone());
            }

            self.current_screen.show(ui, &mut self.state);
        });
    }
}
