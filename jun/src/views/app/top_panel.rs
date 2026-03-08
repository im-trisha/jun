use crate::try_i18n;
use std::fs::{self};

use crate::JunApp;

impl JunApp {
    pub(super) fn top_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");

                ui.menu_button("File", |ui| {
                    if !is_web && ui.button(self.t_topbar_file_quit()).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }

                    if ui.button(self.t_topbar_file_open()).clicked()
                        && let Some(path) = self.mdrg_file_dialog().pick_file()
                    {
                        let content = try_i18n!(
                            self.state,
                            fs::read_to_string(&path),
                            self.t_error_reading_file()
                        );

                        let picked: mdrg::MDRGSaveFile = try_i18n!(
                            self.state,
                            serde_json::from_str(&content),
                            self.t_error_parsing_file()
                        );

                        self.state.working_file = Some(picked);
                        self.state.worked_with.push(path)
                    }

                    if ui.button(self.t_topbar_file_open_recent()).clicked() {}
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_switch(ui);
            });
        });
    }
}
