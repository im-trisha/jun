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
    pub language: Language,
    // A list of files we precedently worked with
    pub worked_with: Vec<PathBuf>,
    /// Already internationalized error string
    #[serde(skip)]
    pub errors: Vec<String>,
    #[serde(skip)]
    pub working_file: Option<MDRGSaveFile>,
    #[serde(skip)]
    working_save_slot: Option<WorkingSaveSlot>,
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
            working_save_slot: Default::default(),
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

        let coll = match slot_info.is_autosave {
            true => &mut save.auto_saves,
            false => &mut save.saves,
        };

        coll.get_mut(slot_info.slot_idx)
    }

    pub fn set_working_save_slot(
        &mut self,
        slot_number: i32,
        is_autosave: bool,
    ) -> Result<(), String> {
        let lang = self.language;
        let save = self
            .working_file
            .as_mut()
            .ok_or(lang.t_screens_file_not_set())?;

        let coll = match is_autosave {
            true => &mut save.auto_saves,
            false => &mut save.saves,
        };

        let idx = coll
            .iter()
            .position(|e| e.slot == slot_number)
            .ok_or(lang.t_screens_invalid_slot())?;

        self.working_save_slot = Some(WorkingSaveSlot {
            slot_idx: idx,
            is_autosave,
        });

        Ok(())
    }
}
