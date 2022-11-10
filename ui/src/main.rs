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
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
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
        egui::Window::new("Plot")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0., 0.))
            // .default_size(egui::vec2(600., 600.))
            // .resizable(true)
            .show(ctx, |ui| {
                use egui::plot::{Bar, BarChart, Legend, Line, Plot, PlotPoints, VLine};
                ui.horizontal(|ui| {
                    let sin: PlotPoints = (0..1000)
                        .map(|i| {
                            let x = i as f64 * 0.01;
                            [x, x.sin()]
                        })
                        .collect();
                    let cos: PlotPoints = (0..1000)
                        .map(|i| {
                            let x = i as f64 * 0.01;
                            [x, x.cos()]
                        })
                        .collect();
                    Plot::new("my_plot")
                        .view_aspect(1.0)
                        .height(150.)
                        .legend(Legend::default()) // with .name() method
                        .include_x(0.) // show x axis label
                        .show(ui, |plot_ui| {
                            plot_ui.vline(VLine::new(0.5));
                            plot_ui.vline(VLine::new(5));
                            plot_ui.vline(VLine::new(9));
                            plot_ui.line(Line::new(sin).name("sin"));
                            plot_ui.line(Line::new(cos).name("cos"));
                        });

                    let bin_size = 5.;
                    let bars = BarChart::new(
                        (0..10)
                            .map(|x| {
                                Bar::new((x as f32 * bin_size) as f64, x as f64)
                                    .width(bin_size as f64)
                            })
                            .collect(),
                    )
                    .color(egui::color::Color32::LIGHT_BLUE);

                    Plot::new("my_barchart")
                        .view_aspect(1.0)
                        .height(150.0)
                        .show(ui, |plot_ui| plot_ui.bar_chart(bars));

                    let points: PlotPoints = [
                        [0.0, 1.0],
                        [1.0, 2.0],
                        [2.0, 5.0],
                        [3.0, 4.0],
                        [4.0, 1.0],
                        [5.0, 2.0],
                    ]
                    .into_iter()
                    .collect();
                    let points: PlotPoints = (0..1000)
                        .map(|i| {
                            let x = i as f64 * 0.01;
                            [x, x.sin()]
                        })
                        .collect();
                    Plot::new("fill line")
                        .view_aspect(1.0)
                        .height(150.)
                        .legend(Legend::default()) // with .name() method
                        .include_x(0.) // show x axis label
                        .show(ui, |plot_ui| {
                            plot_ui.line(Line::new(points).fill(0.).name("fill"));
                        });
                });
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
