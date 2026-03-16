use crate::{
    JunApp,
    views::{
        EmptyScreen, Screens, game_progression::GameProgression, jun_stats::JunStats,
        player_stats::PlayerStats, save_slot_picker::SaveSlotPicker,
    },
};

impl JunApp {
    pub(super) fn side_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("window_options_side_panel").show(ctx, |ui| {
            let lang = self.state.language;

            ui.selectable_value(
                &mut self.current_screen,
                Screens::Empty(EmptyScreen::default()),
                lang.t_screens_empty(),
            );

            if self.state.working_file.is_none() {
                return;
            }

            ui.selectable_value(
                &mut self.current_screen,
                Screens::SaveSlotPicker(SaveSlotPicker::default()),
                lang.t_screens_save_slot_picker_title(),
            );

            if self.state.working_save_slot().is_none() {
                return;
            }

            ui.collapsing(lang.t_screens_collapsable_header(), |ui| {
                ui.selectable_value(
                    &mut self.current_screen,
                    Screens::JunStats(JunStats::default()),
                    lang.t_screens_jun_stats_title(),
                );

                ui.selectable_value(
                    &mut self.current_screen,
                    Screens::PlayerStats(PlayerStats::default()),
                    lang.t_screens_player_stats_title(),
                );

                ui.selectable_value(
                    &mut self.current_screen,
                    Screens::GameProgression(GameProgression::default()),
                    lang.t_screens_game_progression_title(),
                );
            });
        });
    }
}
