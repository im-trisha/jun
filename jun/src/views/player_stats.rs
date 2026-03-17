use crate::{JunAppState, bool_column, heading_column, stat_column, text_column, try_i18n};
use egui::RichText;
use serde::{Deserialize, Serialize};

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct PlayerStats {}

impl PlayerStats {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut JunAppState) {
        let lang = state.language;
        let Some(slot) = state.working_save_slot() else {
            return;
        };
        let data = try_i18n!(state, slot.save_data(), lang.t_error_to_json());

        text_column!(
            ui,
            lang.t_mdrgp_player_name_title(),
            lang.t_mdrgp_player_name_desc(),
            &mut data.player_name
        );

        text_column!(
            ui,
            lang.t_mdrgp_status_text_title(),
            lang.t_mdrgp_status_text_desc(),
            &mut data.status_text
        );

        heading_column!(ui, lang.t_mdrgp_heading_health());
        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_stamina(), &mut data.stamina, 0.0..=1.0);
            stat_column!(cols[1], lang.t_mdrgp_mental_health(), &mut data.mental_health, 0.0..=1.0);
            stat_column!(cols[2], lang.t_mdrgp_mental_health_temporary(), &mut data.mental_health_temporary, 0.0..=1.0);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_satiation(), &mut data.satiation, 0.0..=1.0);
            stat_column!(cols[1], lang.t_mdrgp_health(), &mut data.health, 0.0..=1.0);
            stat_column!(cols[2], lang.t_mdrgp_vinegara_effect_end_title(), lang.t_mdrgp_vinegara_effect_end_desc(), &mut data.vinegara_effect_end, 0..=10);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_death_grip_effect_end_title(), lang.t_mdrgp_death_grip_effect_end_desc(), &mut data.death_grip_effect_end, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_max_cum_title(), lang.t_mdrgp_max_cum_desc(), &mut data.max_cum, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_remaining_cum_title(), lang.t_mdrgp_remaining_cum_desc(), &mut data.remaining_cum, 0..=i32::MAX);
        });

        heading_column!(ui, lang.t_mdrgp_heading_minigames());
        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_times_won_old_maid(), &mut data.times_won_old_maid, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_times_lost_old_maid(), &mut data.times_lost_old_maid, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_times_ran_away_old_maid(), &mut data.times_ran_away_old_maid, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(4, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_times_won_chess(), &mut data.times_won_chess, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_times_lost_chess(), &mut data.times_lost_chess, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_times_won_word_chain(), &mut data.times_won_word_chain, 0..=i32::MAX);
            stat_column!(cols[3], lang.t_mdrgp_times_lost_word_chain(), &mut data.times_lost_word_chain, 0..=i32::MAX);
        });

        heading_column!(ui, lang.t_mdrgp_heading_nasty());
        #[rustfmt::skip]
        ui.columns(4, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_times_came_inside(), &mut data.times_came_inside, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_times_came_inside_anal(), &mut data.times_came_inside_anal, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_times_came_in_mouth(), &mut data.times_came_in_mouth, 0..=i32::MAX);
            stat_column!(cols[3], lang.t_mdrgp_times_came_outside(), &mut data.times_came_outside, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_ml_came_in_mouth(), &mut data.ml_came_in_mouth, 0.0..=f32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_ml_of_cum_wasted_title(), lang.t_mdrgp_ml_of_cum_wasted_desc(), &mut data.ml_of_cum_wasted, 0.0..=f32::MAX);
        });

        heading_column!(ui, lang.t_mdrgp_heading_streaming_economy());
        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_stream_count_title(), lang.t_mdrgp_stream_count_desc(), &mut data.stream_count, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_streamed_for_title(), lang.t_mdrgp_streamed_for_desc(), &mut data.streamed_for, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_money_earned_from_donations_title(), lang.t_mdrgp_money_earned_from_donations_desc(), &mut data.money_earned_from_donations, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_longest_stream_title(), lang.t_mdrgp_longest_stream_desc(), &mut data.longest_stream, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_subs(), &mut data.subs, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_followers(), &mut data.followers, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_money(), &mut data.money, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_casino_tokens(), &mut data.casino_tokens, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_last_streamed_at_title(), lang.t_mdrgp_last_streamed_at_desc(), &mut data.last_streamed_at, 0..=i32::MAX);
        });

        heading_column!(ui, lang.t_mdrgp_heading_time_based_stats());

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_last_worked_at_day(), &mut data.last_worked_at_day, 0..=i32::MAX); // TODO: based on real day
            stat_column!(cols[1], lang.t_mdrgp_last_went_to_church_at_title(), lang.t_mdrgp_last_went_to_church_at_desc(), &mut data.last_went_to_church_at, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_last_cuddled_at_title(), lang.t_mdrgp_last_cuddled_at_desc(), &mut data.last_cuddled_at, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            bool_column!(cols[0], lang.t_mdrgp_last_slept_with_bot_title(), lang.t_mdrgp_last_slept_with_bot_desc(), &mut data.last_slept_with_bot);
            stat_column!(cols[1], lang.t_mdrgp_last_woke_up_at_title(), lang.t_mdrgp_last_woke_up_at_desc(), &mut data.last_woke_up_at, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_last_fucked_at_title(), lang.t_mdrgp_last_fucked_at_desc(), &mut data.last_fucked_at, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_last_interact_at_title(), lang.t_mdrgp_last_interact_at_desc(), &mut data.last_interact_at, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_last_equipment_at_title(), lang.t_mdrgp_last_equipment_at_desc(), &mut data.last_equipment_at, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_last_outside_with_bot_at_title(), lang.t_mdrgp_last_outside_with_bot_at_desc(), &mut data.last_outside_with_bot_at, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_last_headpatted_at_title(), lang.t_mdrgp_last_headpatted_at_desc(), &mut data.last_headpatted_at, 0..=i32::MAX);
            stat_column!(cols[1], lang.t_mdrgp_last_hunger_info_at_title(), lang.t_mdrgp_last_hunger_info_at_desc(), &mut data.last_hunger_info_at, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_last_mental_health_info_at_title(), lang.t_mdrgp_last_mental_health_info_at_desc(), &mut data.last_mental_health_info_at, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], lang.t_mdrgp_last_talked_at_title(), lang.t_mdrgp_last_talked_at_desc(), &mut data.last_talked_at, 0..=i32::MAX);
            stat_column!(cols[2], lang.t_mdrgp_last_bot_started_talk_at_title(), lang.t_mdrgp_last_bot_started_talk_at_desc(), &mut data.last_bot_started_talk_at, 0..=i32::MAX);
        });
    }
}
