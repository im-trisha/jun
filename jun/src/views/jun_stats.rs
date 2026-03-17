use serde::{Deserialize, Serialize};

use crate::{JunAppState, stat_column, text_column, try_i18n};

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct JunStats {}

impl JunStats {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut JunAppState) {
        let Some(slot) = state.working_save_slot() else {
            return;
        };
        let data = try_i18n!(state, slot.save_data(), state.t_error_to_json());

        text_column!(
            ui,
            "Name",
            "Jun is a lovely girlfriend, you'd be a real gentleman keeping her name as such :)",
            &mut data.bot_name
        );

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Lust", &mut data.lust, 0..=i32::MAX);
            stat_column!(cols[1], "Current horniness", "Jun's current arousal level", &mut data.current_horniness, 0.0..=1.0);
            stat_column!(cols[2], "Longing", "Jun's accumulated horniness", &mut data.longing, 0..=10);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Sympathy", "How much she likes you", &mut data.sympathy, 0..=i32::MAX);
            stat_column!(cols[1], "Mood", "Jun's current mood", &mut data.mood, -1.0..=1.0);
            stat_column!(cols[2], "Intelligence", &mut data.intelligence, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Cum inside", "Volume of sperm inside Jun's vagina (mL)", &mut data.cum_inside, 0.0..=f32::MAX);
            stat_column!(cols[1], "Cum inside anal", "Volume of sperm inside Jun's anus (mL)", &mut data.cum_inside_anal, 0.0..=f32::MAX);
            stat_column!(cols[2], "Cum inside stomach", "Volume of sperm inside Jun's stomach (mL)", &mut data.cum_inside_stomach, 0.0..=f32::MAX);
        });
    }
}
