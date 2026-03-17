use crate::{JunAppState, bool_column, heading_column, stat_column, text_column, try_i18n};
use egui::RichText;
use serde::{Deserialize, Serialize};

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct PlayerStats {}

impl PlayerStats {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut JunAppState) {
        let Some(slot) = state.working_save_slot() else {
            return;
        };
        let data = try_i18n!(state, slot.save_data(), state.t_error_to_json());

        text_column!(
            ui,
            "Name",
            "The player's real name is anon",
            &mut data.player_name
        );

        text_column!(
            ui,
            "Status text",
            "The status text shown on the top left, most of the time used for the rent, you can keep this empty",
            &mut data.status_text
        );

        heading_column!(ui, "Health");
        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Stamina", &mut data.stamina, 0.0..=1.0);
            stat_column!(cols[1], "Mental health", &mut data.mental_health, 0.0..=1.0);
            stat_column!(cols[2], "Mental health (temporary)", &mut data.mental_health_temporary, 0.0..=1.0);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Satiation", &mut data.satiation, 0.0..=1.0);
            stat_column!(cols[1], "Health", &mut data.health, 0.0..=1.0);
            stat_column!(cols[2], "Vinegara effect end", "The time (In game minutes) when Vinegara (drug) effect will end", &mut data.vinegara_effect_end, 0..=10);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Death grip effect end", "The time (In game minutes) when Death Grip (drug) effect will end", &mut data.death_grip_effect_end, 0..=i32::MAX);
            stat_column!(cols[1], "Max cum", "Desc", &mut data.max_cum, 0..=i32::MAX);
            stat_column!(cols[2], "Remaining cum", "Desc", &mut data.remaining_cum, 0..=i32::MAX);
        });

        heading_column!(ui, "Minigames");
        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Old maid wins", &mut data.times_won_old_maid, 0..=i32::MAX);
            stat_column!(cols[1], "Old maid losses", &mut data.times_lost_old_maid, 0..=i32::MAX);
            stat_column!(cols[2], "Old maid ran away", &mut data.times_ran_away_old_maid, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(4, |cols| {
            stat_column!(cols[0], "Chess wins", &mut data.times_won_chess, 0..=i32::MAX);
            stat_column!(cols[1], "Chess losses", &mut data.times_lost_chess, 0..=i32::MAX);
            stat_column!(cols[2], "Word chain wins", &mut data.times_won_word_chain, 0..=i32::MAX);
            stat_column!(cols[3], "Word chain losses", &mut data.times_lost_word_chain, 0..=i32::MAX);
        });

        heading_column!(ui, "Nasty");
        #[rustfmt::skip]
        ui.columns(4, |cols| {
            stat_column!(cols[0], "Times came inside (Vagina)", &mut data.times_came_inside, 0..=i32::MAX);
            stat_column!(cols[1], "Times came inside (Anus)", &mut data.times_came_inside_anal, 0..=i32::MAX);
            stat_column!(cols[2], "Times came inside (Mouth)", &mut data.times_came_in_mouth, 0..=i32::MAX);
            stat_column!(cols[3], "Times came outside", &mut data.times_came_outside, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Volume (mL) mouth", &mut data.ml_came_in_mouth, 0.0..=f32::MAX);
            stat_column!(cols[2], "Volume (mL) outside", "Volume (mL) ejaculated anywhere outside of Jun’s orifices", &mut data.ml_of_cum_wasted, 0.0..=f32::MAX);
        });

        heading_column!(ui, "Streaming/Economy");
        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Stream count", "Total number of streams", &mut data.stream_count, 0..=i32::MAX);
            stat_column!(cols[1], "Streamed for", "Total in game minutes spent streaming", &mut data.streamed_for, 0..=i32::MAX);
            stat_column!(cols[2], "Donations money", "Total money earned from stream doations", &mut data.money_earned_from_donations, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Longest stream", "Total in game minutes of the longest stream", &mut data.longest_stream, 0..=i32::MAX);
            stat_column!(cols[1], "Subs", &mut data.subs, 0..=i32::MAX);
            stat_column!(cols[2], "Followers", &mut data.followers, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Money", &mut data.money, 0..=i32::MAX);
            stat_column!(cols[1], "Casino tokens", &mut data.casino_tokens, 0..=i32::MAX);
            stat_column!(cols[2], "Last streamed at", "In-game minute of the last streaming session", &mut data.last_streamed_at, 0..=i32::MAX);
        });

        heading_column!(ui, "Time based stats");

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Last work day", &mut data.last_worked_at_day, 0..=i32::MAX); // TODO: based on real day
            stat_column!(cols[1], "Last church time", "The last time (In game minutes) when you went to church", &mut data.last_went_to_church_at, 0..=i32::MAX);
            stat_column!(cols[2], "Last cuddle time", "The last time (In game minutes) when you cuddled with Jun", &mut data.last_cuddled_at, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            bool_column!(cols[0], "Slept with Jun", "Did you sleep with Jun yet?", &mut data.last_slept_with_bot);
            stat_column!(cols[1], "Last woke up", "The last time (In game minutes) when you woke up", &mut data.last_woke_up_at, 0..=i32::MAX);
            stat_column!(cols[2], "Last intercourse", "The last time (In game minutes) when you had an intercourse with Jun", &mut data.last_fucked_at, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Last interaction", "The last time (In game minutes) when you interacted with Jun", &mut data.last_interact_at, 0..=i32::MAX);
            stat_column!(cols[1], "Last equipment session", "The last time (In game minutes) when you opened the equipment scene", &mut data.last_equipment_at, 0..=i32::MAX);
            stat_column!(cols[2], "Last outside", "The last time (In game minutes) when you were outside with Jun", &mut data.last_outside_with_bot_at, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Last headpat", "The last time (In game minutes) when you headpatted Jun", &mut data.last_headpatted_at, 0..=i32::MAX);
            stat_column!(cols[1], "Last hunger notification", "The last time (In game minutes) when you got an hunger notification", &mut data.last_hunger_info_at, 0..=i32::MAX);
            stat_column!(cols[2], "Last mental notification", "The last time (In game minutes) when you got a mental health notification", &mut data.last_mental_health_info_at, 0..=i32::MAX);
        });

        #[rustfmt::skip]
        ui.columns(3, |cols| {
            stat_column!(cols[0], "Last talk", "The last time (In game minutes) you talked with Jun", &mut data.last_talked_at, 0..=i32::MAX);
            stat_column!(cols[2], "Last bot talk", "The last time (In game minutes) when Jun started a talk with you", &mut data.last_bot_started_talk_at, 0..=i32::MAX);
        });
    }
}
