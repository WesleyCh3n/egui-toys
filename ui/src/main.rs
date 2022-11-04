#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Play around ui",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(PartialEq)]
enum MyEnum {
    Happy,
    Sad,
    Excited,
}
struct MyApp {
    show_window: bool,
    slider_value: u32,
    radio_value: MyEnum,
    progress_value: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_window: false,
            slider_value: 70,
            radio_value: MyEnum::Happy,
            progress_value: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        // TopBottomPanel must before CentralPanel
        egui::TopBottomPanel::top("my panel").show(ctx, |ui| {
            ui.menu_button("My menu", |ui| {
                if ui.button("Close the menu").clicked() {
                    ui.close_menu();
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is heading");
            if ui.button("click button").clicked() {
                self.show_window = true;
            }
            ui.separator();
            ui.group(|ui| {
                ui.label("In group");
                ui.horizontal(|ui| {
                    ui.radio_value(
                        &mut self.radio_value,
                        MyEnum::Happy,
                        "Happy",
                    );
                    ui.radio_value(&mut self.radio_value, MyEnum::Sad, "Sad");
                    ui.radio_value(
                        &mut self.radio_value,
                        MyEnum::Excited,
                        "Excited",
                    );
                });
            });
            ui.group(|ui| {
                ui.label("another group");
                ui.horizontal(|ui| {
                    ui.label("Slider");
                    ui.add(egui::Slider::new(&mut self.slider_value, 0..=100));
                });
                ui.horizontal(|ui| {
                    ui.label("Drager");
                    ui.add(
                        egui::DragValue::new(&mut self.slider_value)
                            .clamp_range(0..=100),
                    );
                });
            });
            ui.weak("somthing\nnextline");
            ui.horizontal(|ui| {
                if ui.button("+").clicked() {
                    self.progress_value += 1;
                }
                if ui.button("-").clicked() {
                    self.progress_value -= 1;
                }
            });
            ui.add(egui::ProgressBar::new(self.progress_value as f32 / 100.));
        });
        egui::Window::new("new window")
            .open(&mut self.show_window)
            .show(ctx, |ui| {
                ui.label("Hello world");
            });
    }
}
