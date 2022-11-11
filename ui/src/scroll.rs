use eframe::egui::{self, ScrollArea};

use crate::MyApp;

pub fn scroll_window(app: &mut MyApp, ui: &mut egui::Ui) {
    ui.label("Below is scroll area");
    let num_rows = 100;
    let mut scroll_top = false;
    let mut scroll_bottom = false;
    ui.horizontal(|ui| {
        scroll_top |= ui.button("Scroll to top").clicked();
        scroll_bottom |= ui.button("Scroll to bottom").clicked();
    });
    ui.add_space(4.);
    ScrollArea::vertical()
        .max_height(200.0)
        .auto_shrink([false; 2])
        .stick_to_bottom(!scroll_top)
        .show(ui, |ui| {
            if scroll_top {
                ui.scroll_to_cursor(Some(egui::Align::TOP));
            }
            for row in 0..100 {
                let text = format!("This is row {}/{}", row + 1, num_rows);
                ui.label(text);
            }
            if scroll_bottom {
                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
            }
        });
}
