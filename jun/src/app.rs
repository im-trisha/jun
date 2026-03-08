use std::{ops::Deref, path::PathBuf};

use mdrg::MDRGSaveFile;
use serde::{Deserialize, Serialize};

use crate::{Language, views::Screens};

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct JunAppState {
    pub language: Language,
    // A list of files we precedently worked with
    pub worked_with: Vec<PathBuf>,
    /// Already internationalized error string
    #[serde(skip)]
    pub errors: Vec<String>,
    #[serde(skip)]
    pub working_file: Option<MDRGSaveFile>,
}

impl Default for JunAppState {
    fn default() -> Self {
        let locales: Vec<_> = sys_locale::get_locales().collect();
        // Gets the first preferred locale
        let language = locales
            .iter()
            .filter_map(|l| Language::from_locale(l))
            .next()
            .unwrap_or(Language::En);

        Self {
            language,
            worked_with: Default::default(),
            errors: Default::default(),
            working_file: Default::default(),
        }
    }
}

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

// Impl deref as language because it has the t_ prefix,
// and its of easier access
impl Deref for JunAppState {
    type Target = Language;

    fn deref(&self) -> &Self::Target {
        &self.language
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
