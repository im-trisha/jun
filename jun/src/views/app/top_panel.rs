use std::path::PathBuf;

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
                        self.load_save(path);
                    }

                    if self.state.recent_paths.is_empty() {
                        return;
                    }

                    ui.menu_button(self.t_topbar_file_open_recent(), |ui| {
                        let mut selected_path: Option<PathBuf> = None;
                        for path in &self.state.recent_paths {
                            if ui.button(path.to_string_lossy()).clicked() {
                                selected_path = Some(path.clone());
                            }
                        }

                        if let Some(path) = selected_path {
                            self.load_save(path.clone());
                            self.add_recent_path(path);
                        }
                    });
                });

                ui.add_space(16.0);

                egui::widgets::global_theme_preference_switch(ui);
            });
        });
    }
}
