use eframe::egui;

use crate::MyApp;

#[derive(PartialEq)]
pub enum MyEnum {
    Happy,
    Sad,
    Excited,
}

pub fn widgets_window(app: &mut MyApp, ui: &mut egui::Ui) {
    ui.heading("This is heading");
    if ui.button("click button").clicked() {
        app.show_window = true;
    }
    ui.separator();
    ui.group(|ui| {
        ui.label("In group");
        ui.horizontal(|ui| {
            ui.radio_value(&mut app.radio_value, MyEnum::Happy, "Happy");
            ui.radio_value(&mut app.radio_value, MyEnum::Sad, "Sad");
            ui.radio_value(&mut app.radio_value, MyEnum::Excited, "Excited");
        });
    });
    ui.group(|ui| {
        ui.label("another group");
        ui.horizontal(|ui| {
            ui.label("Slider");
            ui.add(egui::Slider::new(&mut app.slider_value, 0..=100));
        });
        ui.horizontal(|ui| {
            ui.label("Drager");
            ui.add(
                egui::DragValue::new(&mut app.slider_value)
                    .clamp_range(0..=100),
            );
        });
    });
    ui.horizontal(|ui| {
        ui.set_visible(false);
        if ui.button("start").clicked() {
            long_process(app.sender_data.clone(), 10);
        }
        if ui.button("reset").clicked() {
            *app.process_data.lock().unwrap() = 0;
        }
    });
    ui.add(egui::ProgressBar::new(
        *app.process_data.lock().unwrap() as f32 * (10. / 100.),
    ));
}

fn long_process(sender: std::sync::mpsc::Sender<i32>, length: i32) {
    std::thread::spawn(move || {
        for i in 1..=length {
            sender.send(i).unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
