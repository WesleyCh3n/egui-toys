#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::{Arc, Mutex};

use eframe::egui;
fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::Vec2 { x: 400., y: 110. });
    eframe::run_native(
        "Play around ui",
        options,
        Box::new(|_cc| Box::new(MyApp::new(_cc))),
    );
}

struct MyApp {
    slider_value: u32,
    curr: Arc<Mutex<i32>>,
    sx_curr: std::sync::mpsc::Sender<i32>,
    picked_dir: Option<String>,
    sub_dirs: Vec<std::path::PathBuf>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (sx_data, rx) = std::sync::mpsc::channel();
        let result = Arc::new(Mutex::new(0));
        spawn_repaint_thread(rx, result.clone(), cc.egui_ctx.clone());

        Self {
            slider_value: 70,
            curr: result,
            sx_curr: sx_data,
            picked_dir: None,
            sub_dirs: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Open Dir…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.picked_dir = Some(path.display().to_string());
                            self.sub_dirs = std::fs::read_dir(path)
                                .unwrap()
                                .into_iter()
                                .filter(|d| d.as_ref().unwrap().metadata().unwrap().is_dir())
                                .map(|d| d.unwrap().path())
                                .collect();
                        }
                    }
                    ui.vertical_centered_justified(|ui| {
                        if let Some(picked_dir) = &self.picked_dir {
                            ui.strong(picked_dir);
                        } else {
                            ui.label("");
                        }
                    });
                });
                ui.vertical_centered_justified(|ui| {
                    ui.style_mut().spacing.slider_width = 300.;
                    ui.add(egui::Slider::new(&mut self.slider_value, 0..=100).text("%"));
                })
            });
            ui.vertical_centered(|ui| {
                let mut data = *self.curr.lock().unwrap();
                if data == self.sub_dirs.len() as i32 {
                    data = 0;
                }
                ui.add_visible_ui(data != 0, |ui| {
                    ui.add(egui::ProgressBar::new(
                        data as f32 / self.sub_dirs.len() as f32,
                    ));
                });
                ui.add_enabled_ui(data == 0 && self.picked_dir.is_some(), |ui| {
                    if ui.button(" Start ").clicked() {
                        long_process(self.sx_curr.clone(), self.sub_dirs.clone());
                    }
                });
            })
        });
    }
}

fn long_process(sender: std::sync::mpsc::Sender<i32>, dirs: Vec<std::path::PathBuf>) {
    std::thread::spawn(move || {
        for (i, dir) in dirs.iter().enumerate() {
            sender.send(i as i32 + 1).unwrap();
            println!("{:?}", dir);
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
