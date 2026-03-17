use serde::{Deserialize, Serialize};
use std::{fs, ops::Deref, path::PathBuf};

use crate::{JunAppState, Language, try_i18n, views::Screens};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct JunApp {
    pub state: JunAppState,
    #[serde(skip)]
    pub current_screen: Screens,
}

// Impl deref as language because it has the t_ prefix,
// and its of easier access
impl Deref for JunApp {
    type Target = Language;

    fn deref(&self) -> &Self::Target {
        &self.state.language
    }
}

impl JunApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        egui_material_icons::initialize(&cc.egui_ctx);

        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn mdrg_file_dialog(&self) -> rfd::FileDialog {
        rfd::FileDialog::new().add_filter(self.t_fd_mdrg_filetype(), &["mdrg"])
    }

    pub fn load_save(&mut self, path: PathBuf) {
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

        self.state.unset_working_save_slot();

        self.state.working_file = Some(picked);
        self.add_recent_path(path);
    }

    pub fn export_save(&mut self, path: PathBuf) {
        if let Some(data) = self.state.working_save_slot() {
            try_i18n!(self.state, data.flush_data(), self.t_error_to_json());
        }

        let Some(save) = self.state.working_file.as_ref() else {
            return;
        };

        let content = try_i18n!(
            self.state,
            serde_json::to_string_pretty(save),
            self.t_error_to_json()
        );

        try_i18n!(
            self.state,
            fs::write(&path, content),
            self.t_error_writing_file()
        );

        self.add_recent_path(path);
    }

    pub fn add_recent_path(&mut self, path: PathBuf) {
        if let Some(pos) = self.state.worked_with.iter().position(|x| *x == path) {
            self.state.worked_with.remove(pos);
        }
        self.state.worked_with.insert(0, path);

        while self.state.worked_with.len() > 8 {
            self.state.worked_with.pop();
        }
    }
}
