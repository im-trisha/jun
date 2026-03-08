use crate::{
    JunApp,
    views::{EmptyScreen, Screens, save_slot_picker::SaveSlotPicker},
};

impl JunApp {
    pub(super) fn side_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("window_options_side_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let lang = self.state.language;

                ui.selectable_value(
                    &mut self.current_screen,
                    Screens::Empty(EmptyScreen::default()),
                    lang.t_screens_empty(),
                );

                if self.state.working_file.is_some() {
                    ui.selectable_value(
                        &mut self.current_screen,
                        Screens::SaveSlotPicker(SaveSlotPicker::default()),
                        lang.t_screens_save_slot_picker_title(),
                    );
                }
            });
        });
    }
}
