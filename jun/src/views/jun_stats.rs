use serde::{Deserialize, Serialize};

use crate::{JunAppState, stat_column, text_column, try_i18n};

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct JunStats {}

impl JunStats {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut JunAppState) {
        let lang = state.language;
        let Some(slot) = state.working_save_slot() else {
            return;
        };
        let data = try_i18n!(state, slot.save_data(), lang.t_error_to_json());

        text_column!(
            ui,
            lang.t_mdrgp_bot_name_title(),
            lang.t_mdrgp_bot_name_desc(),
            &mut data.bot_name
        );

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_lust(), &mut data.lust, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_current_horniness_title(), lang.t_mdrgp_current_horniness_desc(), &mut data.current_horniness, 0.0..=1.0);
            stat_column!(cols[2], lang.t_mdrgp_longing_title(), lang.t_mdrgp_longing_desc(), &mut data.longing, 0..=10);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_sympathy_title(), lang.t_mdrgp_sympathy_desc(), &mut data.sympathy, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_mood_title(), lang.t_mdrgp_mood_desc(), &mut data.mood, -1.0..=1.0);
            stat_column!(cols[2], lang.t_mdrgp_intelligence(), &mut data.intelligence, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_cum_inside_title(), lang.t_mdrgp_cum_inside_desc(), &mut data.cum_inside, 0.0..=f32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_cum_inside_anal_title(), lang.t_mdrgp_cum_inside_anal_desc(), &mut data.cum_inside_anal, 0.0..=f32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_cum_inside_stomach_title(), lang.t_mdrgp_cum_inside_stomach_desc(), &mut data.cum_inside_stomach, 0.0..=f32::MAX);
        });
    }
}
