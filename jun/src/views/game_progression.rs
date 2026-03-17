use serde::{Deserialize, Serialize};

use crate::JunAppState;

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct GameProgression {}

// TODO: maybe pub story_flags: Vec<Flag>,
impl GameProgression {
    pub fn ui(&mut self, _ui: &mut egui::Ui, _state: &mut JunAppState) {
        // light_switch_on bool
        // stage i32
        // unique_conversations_left i32
        // search f32
        // priest_bot_points i32
        // nun_points i32
        // weekly_rent i32
        // story_flags Vec<Flag>
        // times_went_to_church i32
    }
}
