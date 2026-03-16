use egui::{RichText, Ui};
use serde::{Deserialize, Serialize};

use crate::{JunAppState, bool_column, stat_column, text_column, try_i18n};

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct PlayerStats {}

impl PlayerStats {
    fn heading(ui: &mut Ui, text: impl Into<String>) {
        ui.vertical(|ui| {
            ui.add_space(32.);
            ui.label(RichText::new(text).size(32.));
            ui.add_space(32.);
        });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut JunAppState) {
        let Some(slot) = state.working_save_slot() else {
            return;
        };
        let data = try_i18n!(state, slot.save_data(), state.t_error_to_json());

        egui::ScrollArea::vertical().show(ui, |ui| {
                text_column!(
                    ui,
                    "Name",
                    "The player's real name is anon",
                    &mut data.player_name
                );

                text_column!(ui, "Title", "Desc", &mut data.status_text);

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.last_worked_at_day, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.last_went_to_church_at, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.last_cuddled_at, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    bool_column!(cols[0], "Title", "Desc", &mut data.last_slept_with_bot);
                    stat_column!(cols[1], "Title", "Desc", &mut data.last_woke_up_at, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.last_fucked_at, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.last_interact_at, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.last_equipment_at, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.last_outside_with_bot_at, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.last_streamed_at, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.last_talked_at, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.last_bot_started_talk_at, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.last_headpatted_at, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.last_hunger_info_at, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.last_mental_health_info_at, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.subs, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.followers, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.money, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.casino_tokens, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.max_cum, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.remaining_cum, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.stamina, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.mental_health, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.mental_health_temporary, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.satiation, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.health, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.vinegara_effect_end, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.death_grip_effect_end, 0..=i32::MAX);
                    stat_column!(cols[1], "Title", "Desc", &mut data.times_lost_chess, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.times_won_chess, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.times_lost_old_maid, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.times_won_old_maid, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.times_ran_away_old_maid, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.times_lost_word_chain, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.times_won_word_chain, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.times_came_inside, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.times_came_inside_anal, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.times_came_outside, 0..=10);
                    stat_column!(cols[2], "Title", "Desc", &mut data.times_came_in_mouth, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.ml_came_in_mouth, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.ml_of_cum_wasted, 0..=10);
                });

                Self::heading(ui, "Streaming");

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.stream_count, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.streamed_for, 0..=10);
                });

                #[rustfmt::skip]
                ui.columns(3, |cols| {
                    stat_column!(cols[0], "Title", "Desc", &mut data.money_earned_from_donations, 0..=10);
                    stat_column!(cols[1], "Title", "Desc", &mut data.longest_stream, 0..=10);
                });
        });
    }
}
