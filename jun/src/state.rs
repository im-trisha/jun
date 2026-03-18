use std::{ops::Deref, path::PathBuf};

use mdrg::{MDRGSaveFile, MDRGSaveRecord};
use serde::{Deserialize, Serialize};

use crate::Language;

pub struct WorkingSaveSlot {
    pub slot_idx: usize,
    pub is_autosave: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct JunAppState {
    /// The selected language
    pub language: Language,
    /// A list of files we precedently worked with
    pub recent_paths: Vec<PathBuf>,
    /// If the user is a poweruser and wants to see settings that possibly break the save file
    pub godmode: bool,
    /// Show/don't show Annalie and Shanice in the background
    pub freaky: bool,
    /// Already internationalized error string
    #[serde(skip)]
    pub errors: Vec<String>,
    #[serde(skip)]
    pub working_file: Option<MDRGSaveFile>,
    #[serde(skip)]
    working_save_slot: Option<WorkingSaveSlot>,
    #[serde(skip)]
    pub show_about: bool,
}

impl Default for JunAppState {
    fn default() -> Self {
        let locales: Vec<_> = sys_locale::get_locales().collect();
        // Gets the first preferred locale
        let language = locales
            .iter()
            .find_map(|l| Language::from_locale(l))
            .unwrap_or(Language::En);

        Self {
            language,
            godmode: false,
            show_about: false,
            freaky: true,
            recent_paths: Vec::default(),
            errors: Vec::default(),
            working_file: Option::default(),
            working_save_slot: Option::default(),
        }
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

impl JunAppState {
    pub fn working_save_slot(&mut self) -> Option<&mut MDRGSaveRecord> {
        let save = self.working_file.as_mut()?;
        let slot_info = self.working_save_slot.as_ref()?;

        let coll = if slot_info.is_autosave {
            &mut save.auto_saves
        } else {
            &mut save.saves
        };

        coll.get_mut(slot_info.slot_idx)
    }

    pub const fn unset_working_save_slot(&mut self) {
        self.working_save_slot = None;
    }

    /// Sets the working save slot
    ///
    /// # Errors
    ///
    /// Returns a localized error if something occurs
    pub fn set_working_save_slot(
        &mut self,
        slot_number: i32,
        is_autosave: bool,
    ) -> Result<(), String> {
        let lang = self.language;

        // Before reassigning, we flush the old save
        if let Some(old) = self.working_save_slot() {
            old.flush_data().map_err(|_| lang.t_error_to_json())?;
        }

        let save = self
            .working_file
            .as_mut()
            .ok_or_else(|| lang.t_error_file_not_set())?;

        let coll = if is_autosave {
            &mut save.auto_saves
        } else {
            &mut save.saves
        };

        let idx = coll
            .iter()
            .position(|e| e.slot == slot_number)
            .ok_or_else(|| lang.t_error_invalid_slot())?;

        self.working_save_slot = Some(WorkingSaveSlot {
            slot_idx: idx,
            is_autosave,
        });

        Ok(())
    }
}
