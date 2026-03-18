use serde::{Deserialize, Serialize};

use crate::{JunAppState, views::app::ScreenView};

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct EmptyScreen {}

impl ScreenView for EmptyScreen {
    fn ui(&mut self, _ui: &mut egui::Ui, _state: &mut JunAppState) {}
}
