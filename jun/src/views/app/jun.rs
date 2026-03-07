use crate::JunApp;

impl eframe::App for JunApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        let error_label = self.t_error_label();
        let ok_label = self.t_ok();
        self.errors.retain(|error| {
            let mut keep = true;
            let win_id = format!("{:p}", error as *const _);
            egui::Window::new(error_label)
                .id(win_id.into())
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(error);

                    if ui.button(ok_label).clicked() {
                        keep = false;
                    }
                });

            keep
        });

        self.side_panel(ctx, frame);
        self.top_panel(ctx, frame);
        self.content(ctx, frame);
    }
}
