use std::ops::Deref;

use mdrg::MDRGSaveFile;

use crate::Language;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct JunApp {
    #[serde(skip)]
    pub language: Language,
    pub worked_with: Vec<String>,
    /// Already internationalized error string
    #[serde(skip)]
    pub errors: Vec<String>,
    #[serde(skip)]
    pub working_file: Option<MDRGSaveFile>,
}

impl Deref for JunApp {
    type Target = Language;

    fn deref(&self) -> &Self::Target {
        &self.language
    }
}

impl Default for JunApp {
    fn default() -> Self {
        let locales: Vec<_> = sys_locale::get_locales().collect();
        // Gets the first preferred locale
        let language = locales
            .iter()
            .map(|l| Language::from_locale(l))
            .next()
            .unwrap_or(Language::En);

        Self {
            language,
            worked_with: Vec::new(),
            errors: Vec::new(),
            working_file: None,
        }
    }
}

impl JunApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    pub fn mdrg_file_dialog(&self) -> rfd::FileDialog {
        rfd::FileDialog::new().add_filter(self.t_fd_mdrg_filetype(), &["mdrg"])
    }
}
