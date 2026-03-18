use std::num::NonZeroUsize;

use egui::{
    Align, Button, Color32, Frame, Layout, Margin, Response, RichText, Shape, Stroke, TextEdit, Ui,
    Widget, vec2,
};
use mdrg::{MDRGSaveFile, MDRGSaveRecord};
use serde::{Deserialize, Serialize};

use crate::{JunAppState, Language};

#[derive(Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
enum SlotTabs {
    Numbered(NonZeroUsize),
    AutoSave,
}

impl Default for SlotTabs {
    fn default() -> Self {
        Self::Numbered(NonZeroUsize::new(1).expect("Math doesn't work, 1 is <= 0"))
    }
}

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct SaveSlotPicker {
    tab: SlotTabs,
}

enum SlotAction {
    Select,
    Delete,
}

impl SaveSlotPicker {
    fn with_triangle(ui: &Ui, res: Response) -> Response {
        const SIZE: f32 = 6.0;

        let base_pos = res.rect.left_bottom();
        let points = vec![
            base_pos,
            base_pos + vec2(SIZE, 0.0),
            base_pos + vec2(0.0, -SIZE),
        ];

        let shape = Shape::convex_polygon(points, Color32::WHITE, Stroke::NONE);
        ui.painter().add(shape);

        res
    }

    fn fancy_tab_btn(&mut self, ui: &mut Ui, value: SlotTabs, text: String) -> Response {
        ui.scope(|ui| {
            // TODO: can't make the non selected colored :(
            let response = Button::selectable(self.tab == value, text)
                .fill(Color32::PURPLE)
                .ui(ui);

            let mut response = Self::with_triangle(ui, response);

            if response.clicked() && self.tab != value {
                self.tab = value;
                response.mark_changed();
            }

            response
        })
        .inner
    }

    fn show_tab_bar(&mut self, ui: &mut Ui, save_file: &MDRGSaveFile, lang: Language) {
        ui.horizontal(|ui| {
            let count = save_file.saves.len().div_ceil(7);
            for i in 1..=count {
                let v = NonZeroUsize::new(i).expect("Math doesn't work, or the for cycle is bad");
                self.fancy_tab_btn(ui, SlotTabs::Numbered(v), i.to_string());
            }

            self.fancy_tab_btn(
                ui,
                SlotTabs::AutoSave,
                lang.t_screens_save_slot_picker_autosave().into(),
            );
        });
    }

    fn save_slot(
        ui: &mut Ui,
        save: &mut mdrg::MDRGSaveRecord,
        lang: Language,
    ) -> Option<SlotAction> {
        let mut action_taken = None;

        let frame = Frame::new()
            .fill(Color32::from_rgba_unmultiplied(0x80, 0, 0x80, 0xEE))
            .inner_margin(Margin::same(10))
            .outer_margin(Margin::same(10));

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(lang.t_screens_save_slot_picker_slot(
                            &save.slot.to_string(),
                            &save.ingame_time.to_string(),
                        ));

                        ui.label(
                            RichText::new(
                                lang.t_screens_save_slot_picker_created_at(&save.time.to_string()),
                            )
                            .weak(),
                        );
                    });

                    let text_edit = TextEdit::singleline(&mut save.notes);
                    let text_edit = text_edit.hint_text(lang.t_screens_save_slot_picker_no_note());
                    let text_edit = text_edit.background_color(Color32::TRANSPARENT);

                    ui.add(text_edit);
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.horizontal(|ui| {
                        let button = ui.add(Button::new(lang.t_screens_save_slot_picker_select()));

                        if Self::with_triangle(ui, button).clicked() {
                            action_taken = Some(SlotAction::Select);
                        }

                        let button = ui.button(egui_material_icons::icons::ICON_DELETE);
                        if Self::with_triangle(ui, button).clicked() {
                            action_taken = Some(SlotAction::Delete);
                        }
                    });
                });
            });
        });

        action_taken
    }

    pub fn ui(&mut self, ui: &mut Ui, state: &mut JunAppState) {
        let Some(save_file) = state.working_file.as_mut() else {
            return;
        };

        self.show_tab_bar(ui, save_file, state.language);

        ui.separator();

        let (selected_slot, auto_save) = egui::ScrollArea::vertical()
            .show(ui, |ui| match self.tab {
                SlotTabs::Numbered(v) => {
                    let end = v.get() * 7;
                    let start = end.saturating_sub(7);
                    let slot =
                        Self::process_slots(ui, state.language, &mut save_file.saves, start..end);
                    (slot, false)
                }
                SlotTabs::AutoSave => {
                    let len = save_file.auto_saves.len();
                    let slot =
                        Self::process_slots(ui, state.language, &mut save_file.auto_saves, 0..len);
                    (slot, true)
                }
            })
            .inner;

        if let Some(slot) = selected_slot
            && let Err(e) = state.set_working_save_slot(slot, auto_save)
        {
            state.errors.push(e);
        }
    }
    fn process_slots(
        ui: &mut Ui,
        language: Language,
        saves: &mut Vec<MDRGSaveRecord>,
        indices: impl Iterator<Item = usize>,
    ) -> Option<i32> {
        let mut delete_index = None;
        let mut selected_slot = None;

        for i in indices {
            let Some(save) = saves.get_mut(i) else {
                continue;
            };

            match Self::save_slot(ui, save, language) {
                Some(SlotAction::Delete) => delete_index = Some(i),
                Some(SlotAction::Select) => selected_slot = Some(save.slot),
                None => {}
            }

            ui.separator();
        }

        if let Some(idx) = delete_index {
            saves.remove(idx);
        }

        selected_slot
    }
}
