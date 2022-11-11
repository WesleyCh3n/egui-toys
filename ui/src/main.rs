#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod plots;
mod scroll;
mod widgets;
use std::sync::{Arc, Mutex};

use eframe::egui;
use plots::{plots_window, PlotPanel};
use scroll::scroll_window;
use widgets::{widgets_window, MyEnum};
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Play around ui",
        options,
        Box::new(|_cc| Box::new(MyApp::new(_cc))),
    );
}

pub struct MyApp {
    show_window: bool,
    slider_value: u32,
    radio_value: MyEnum,
    process_data: Arc<Mutex<i32>>,
    sender_data: std::sync::mpsc::Sender<i32>,
    open_plot: PlotPanel,
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
            open_plot: PlotPanel::Lines,
        }
    }
}

impl eframe::App for MyApp {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    ) {
        // process event
        for e in &ctx.input().events {
            match e {
                // keyboard events
                egui::Event::Key {
                    key,
                    pressed: _,
                    modifiers,
                } => {
                    if modifiers.ctrl == true {
                        match key {
                            egui::Key::W => frame.close(),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        // TopBottomPanel must before CentralPanel
        egui::TopBottomPanel::top("my panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {}
                    if ui.button("Close").clicked() {}
                });
                ui.menu_button("Edits", |_ui| {});
                ui.menu_button("Tools", |_ui| {});
            })
        });

        // main panel
        egui::CentralPanel::default().show(ctx, |_| {});
        egui::Window::new("new window")
            .open(&mut self.show_window)
            .show(ctx, |ui| {
                ui.label("Hello world");
            });
        egui::Window::new("Widgets")
            // .anchor(egui::Align2::LEFT_TOP, egui::vec2(0., 0.))
            .default_pos(egui::pos2(120., 0.))
            .default_size(egui::vec2(100., 0.))
            .show(ctx, |ui| {
                widgets_window(self, ui);
            });
        egui::Window::new("Plot")
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(0., 0.))
            .collapsible(false)
            .default_size(egui::vec2(400., 300.))
            .show(ctx, |ui| {
                plots_window(self, ui);
            });
        egui::Window::new("Scroll")
            // .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(0., 0.))
            .default_pos(egui::pos2(120., 300.))
            .collapsible(false)
            .default_size(egui::vec2(200., 200.))
            .min_width(200.)
            .show(ctx, |ui| {
                scroll_window(self, ui);
            });
        egui::SidePanel::left("side panel").show(ctx, |ui| {
            ui.label("HI");
        });
    }
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
