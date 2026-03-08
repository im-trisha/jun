use crate::{
    JunAppState, Language,
    views::{empty_screen::EmptyScreen, save_slot_picker::SaveSlotPicker},
};
use serde::{Deserialize, Serialize};
mod content;
mod jun;
mod side_panel;
mod top_panel;

#[derive(Eq, PartialEq, Deserialize, Serialize)]
pub enum Screens {
    Empty(EmptyScreen),
    SaveSlotPicker(SaveSlotPicker),
}

impl Default for Screens {
    fn default() -> Self {
        Self::Empty(EmptyScreen::default())
    }
}

impl Screens {
    pub fn title(&self, language: Language) -> String {
        let s = match self {
            Screens::Empty(_) => language.t_screens_empty(),
            Screens::SaveSlotPicker(_) => language.t_screens_save_slot_picker_title(),
        };
        s.into()
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut JunAppState) {
        match &mut *self {
            Screens::Empty(empty_screen) => empty_screen.ui(ui, state),
            Screens::SaveSlotPicker(save_slot_picker) => save_slot_picker.ui(ui, state),
        }
    }
}
