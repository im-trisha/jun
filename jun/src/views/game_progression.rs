use crate::{JunAppState, bool_column, heading_column, stat_column, try_i18n};
use egui::RichText;
use serde::{Deserialize, Serialize};

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct GameProgression {}

// TODO: maybe pub story_flags: Vec<Flag>,
impl GameProgression {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut JunAppState) {
        let lang = state.language;
        let godmode = state.godmode;
        let Some(slot) = state.working_save_slot() else {
            return;
        };
        let data = try_i18n!(state, slot.save_data(), lang.t_error_to_json());

        heading_column!(ui, lang.t_mdrgp_heading_jun_and_apt());
        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_unique_conversations_left_title(), lang.t_mdrgp_unique_conversations_left_desc(), &mut data.unique_conversations_left, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_weekly_rent(), &mut data.weekly_rent, 0..=i32::MAX);
            bool_column!(cols[2], lang.t_mdrgp_room_light_title(), lang.t_mdrgp_room_light_desc(), &mut data.light_switch_on);
        });

        if godmode {
            #[rustfmt::skip]
            ui.columns(3, |cols| {
                stat_column!(cols[0], lang.t_mdrgp_search_suspects_title(), lang.t_mdrgp_search_suspects_desc(), &mut data.search, 0.0..=1.0);
                stat_column!(cols[2], lang.t_mdrgp_relationship_stage_title(), lang.t_mdrgp_relationship_stage_desc(), &mut data.stage, 0..=16);
            });
        }

        heading_column!(ui, lang.t_mdrgp_heading_church());

        let error_color = ui.style().visuals.error_fg_color;
        ui.colored_label(error_color, lang.t_mdrgp_heading_church_subtitle());

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_priestbot_points(), &mut data.priest_bot_points, 0..=i32::MAX);
            if godmode {
                stat_column!(cols[1], lang.t_mdrgp_times_went_to_church(),  &mut data.times_went_to_church, 0..=i32::MAX);
            }
            stat_column!(cols[2], lang.t_mdrgp_nun_points(), &mut data.nun_points, 0..=i32::MAX);
        });
    }
}
