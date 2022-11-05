#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::{Arc, Mutex};

use eframe::egui;
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Play around ui",
        options,
        Box::new(|_cc| Box::new(MyApp::new(_cc))),
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
    process_data: Arc<Mutex<i32>>,
    sender_data: std::sync::mpsc::Sender<i32>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (sx_data, rx) = std::sync::mpsc::channel();
        let result = Arc::new(Mutex::new(0));
        spawn_repaint_thread(rx, result.clone(), cc.egui_ctx.clone());

        Self {
            show_window: false,
            slider_value: 70,
            radio_value: MyEnum::Happy,
            process_data: result,
            sender_data: sx_data,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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
                    ui.radio_value(&mut self.radio_value, MyEnum::Happy, "Happy");
                    ui.radio_value(&mut self.radio_value, MyEnum::Sad, "Sad");
                    ui.radio_value(&mut self.radio_value, MyEnum::Excited, "Excited");
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
                    ui.add(egui::DragValue::new(&mut self.slider_value).clamp_range(0..=100));
                });
            });
            ui.weak("somthing\nnextline");
            ui.horizontal(|ui| {
                ui.set_visible(false);
                if ui.button("start").clicked() {
                    long_process(self.sender_data.clone(), 10);
                }
                if ui.button("reset").clicked() {
                    *self.process_data.lock().unwrap() = 0;
                }
            });
            ui.add(egui::ProgressBar::new(
                *self.process_data.lock().unwrap() as f32 * (10. / 100.),
            ));
        });
        egui::Window::new("new window")
            .open(&mut self.show_window)
            .show(ctx, |ui| {
                ui.label("Hello world");
            });
    }
}

fn long_process(sender: std::sync::mpsc::Sender<i32>, length: i32) {
    std::thread::spawn(move || {
        for i in 1..=length {
            sender.send(i).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

fn spawn_repaint_thread<T: std::marker::Send + 'static>(
    rx: std::sync::mpsc::Receiver<T>,
    data: Arc<Mutex<T>>,
    ctx: egui::Context,
) {
    std::thread::spawn(move || loop {
        if let Ok(a) = rx.recv() {
            let mut data_ = data.lock().unwrap();
            *data_ = a;
            ctx.request_repaint();
        }
    });
}
