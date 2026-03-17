use std::path::PathBuf;

use egui::Ui;

use crate::{JunApp, Language};

impl JunApp {
    fn file(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        // NOTE: no File->Quit on web pages!
        let is_web = cfg!(target_arch = "wasm32");

        ui.menu_button(self.t_topbar_file_label(), |ui| {
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
    }

    fn settings(&mut self, ui: &mut Ui) {
        ui.menu_button(self.t_topbar_settings_label(), |ui| {
            ui.menu_button(self.t_topbar_settings_language_label(), |ui| {
                for lang in Language::VALUES {
                    ui.selectable_value(&mut self.state.language, *lang, lang.name());
                }
            });

            let lang = self.state.language;
            ui.checkbox(&mut self.state.godmode, lang.t_topbar_settings_godmode());
        });
    }

    fn about(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        if ui.button(self.t_topbar_about_label()).clicked() {
            self.state.show_about = true;
        }

        let lang = self.state.language;
        egui::Window::new(self.t_topbar_about_label())
            .open(&mut self.state.show_about)
            .show(ctx, |ui| {
                ui.label(lang.t_topbar_about_body());
            });
    }

    pub(super) fn top_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                self.file(ctx, ui);
                self.settings(ui);
                self.about(ctx, ui);
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_switch(ui);
            });
        });
    }
}
