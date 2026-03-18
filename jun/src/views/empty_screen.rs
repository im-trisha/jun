use serde::{Deserialize, Serialize};

use crate::{JunAppState, views::app::ScreenView};

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct EmptyScreen {}

impl ScreenView for EmptyScreen {
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut JunAppState) {
        ui.heading(state.t_screens_empty_heading());
        ui.label(state.t_screens_empty_body());
    }
}
