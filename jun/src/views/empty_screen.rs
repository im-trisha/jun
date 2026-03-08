use serde::{Deserialize, Serialize};

use crate::JunAppState;

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct EmptyScreen {}

impl EmptyScreen {
    pub fn ui(&mut self, _ui: &mut egui::Ui, _state: &mut JunAppState) {}
}
