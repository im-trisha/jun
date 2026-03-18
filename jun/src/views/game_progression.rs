use std::ops::IndexMut;

use crate::{JunAppState, Language, bool_column, heading_column, stat_column, try_i18n};
use egui::{RichText, Ui};
use mdrg::{MDRGSaveSlot, save::StoryFlags};
use serde::{Deserialize, Serialize};

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct GameProgression {}

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

        if godmode {
            self.story_flags(ui, lang, data);
        }
    }

    fn story_flags(&self, ui: &mut Ui, lang: Language, data: &mut MDRGSaveSlot) {
        heading_column!(ui, lang.t_mdrgp_heading_story_flags());

        for chunk in StoryFlags::KNOWN_FLAGS.chunks(4) {
            ui.columns(4, |cols| {
                for (i, flag) in chunk.iter().enumerate() {
                    let flag_idx = data.story_flags.iter().position(|e| &e.name == flag);
                    let mut checked = flag_idx.is_some();

                    cols[i].vertical(|ui| {
                        if ui.checkbox(&mut checked, *flag).clicked() {
                            if let Some(idx) = flag_idx {
                                data.story_flags.swap_remove(idx);
                            } else {
                                let storyflag = StoryFlags::new(flag.to_string(), data.time);
                                data.story_flags.push(storyflag);
                            }
                        }

                        let Some(flag) = flag_idx.map(|e| data.story_flags.index_mut(e)) else {
                            return;
                        };

                        ui.label(lang.t_mdrgp_story_flags_times_set());
                        ui.add(
                            egui::DragValue::new(&mut flag.times)
                                .speed(0.1)
                                .range(1..=i32::MAX),
                        );
                        stat_column!(ui, lang.t_mdrgp_nun_points(), &mut flag.times, 0..=i32::MAX);
                        ui.add_space(8.0);
                    });
                }
            });
        }
    }
}
