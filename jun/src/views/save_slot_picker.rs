use std::num::NonZeroU8;

use egui::{Color32, Response, Shape, Stroke, Widget, vec2};
use serde::{Deserialize, Serialize};

use crate::{JunAppState, Language};

#[derive(Clone, Copy, Eq, PartialEq, Deserialize, Serialize)]
enum SlotTabs {
    Numbered(NonZeroU8),
    AutoSave,
}

impl Default for SlotTabs {
    fn default() -> Self {
        SlotTabs::Numbered(NonZeroU8::new(1).expect("Math doesn't work, 1 is <= 0"))
    }
}

#[derive(Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct SaveSlotPicker {
    tab: SlotTabs,
}

impl SaveSlotPicker {
    fn with_triangle(ui: &mut egui::Ui, res: Response) -> Response {
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

    fn fancy_tab_btn(&mut self, ui: &mut egui::Ui, value: SlotTabs, text: String) -> Response {
        ui.scope(|ui| {
            // TODO: can't make the non selected colored :(
            let response = egui::Button::selectable(self.tab == value, text)
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

    fn show_tab_bar(&mut self, ui: &mut egui::Ui, lang: Language) {
        ui.horizontal(|ui| {
            for i in 1..=15 {
                let v = NonZeroU8::new(i).expect("Math doesn't work, or the for cycle is bad");
                self.fancy_tab_btn(ui, SlotTabs::Numbered(v), i.to_string());
            }

            self.fancy_tab_btn(
                ui,
                SlotTabs::AutoSave,
                lang.t_screens_save_slot_picker_autosave().into(),
            );
        });
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut JunAppState) {
        self.show_tab_bar(ui, state.language);

        ui.separator();

        match self.tab {
            SlotTabs::Numbered(v) => {
                ui.label("General settings");
            }
            SlotTabs::AutoSave => {
                ui.label("General settings");
            }
        }
    }
}
