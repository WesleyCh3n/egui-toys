use eframe::egui::{self, plot::Polygon};

use crate::MyApp;
use egui::plot::{
    Bar, BarChart, BoxElem, BoxPlot, BoxSpread, Legend, Line, Plot, PlotPoints,
    VLine,
};

#[derive(PartialEq)]
pub enum PlotPanel {
    Lines,
    LineFill,
    Bars,
    BoxPlot,
}

pub fn plots_window(app: &mut MyApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut app.open_plot, PlotPanel::Lines, "Lines");
        ui.selectable_value(
            &mut app.open_plot,
            PlotPanel::LineFill,
            "LineFill",
        );
        ui.selectable_value(&mut app.open_plot, PlotPanel::Bars, "Bars");
        ui.selectable_value(&mut app.open_plot, PlotPanel::BoxPlot, "BoxPlot");
    });
    ui.separator();

    match app.open_plot {
        PlotPanel::Lines => {
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
                .legend(Legend::default()) // with .name() method
                .include_x(0.) // show x axis label
                .show(ui, |plot_ui| {
                    plot_ui.vline(VLine::new(0.5));
                    plot_ui.vline(VLine::new(5));
                    plot_ui.vline(VLine::new(9));
                    plot_ui.line(Line::new(sin).name("sin"));
                    plot_ui.line(Line::new(cos).name("cos"));

                    use std::f64::consts::PI;
                    plot_ui.polygon(Polygon::new(
                        PlotPoints::from_parametric_callback(
                            |t| {
                                let w = 4.;
                                let h = 3.;
                                if 0. <= t && t < PI / 4. {
                                    return (h * t.tan(), h);
                                } else if PI / 4. <= t && t < PI / 2. {
                                    return (w, w * t.tan());
                                } else if PI / 2. <= t && t < PI * 3. / 4. {
                                    return (w, -w * t.tan());
                                }
                                return (t.cos(), t.sin());
                            },
                            0.0..2. * PI,
                            360,
                        ),
                    ));
                });
        }
        PlotPanel::LineFill => {
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
            let egui::InnerResponse {
                response: _,
                inner: (pointer_coordinate, _plot_clicked),
            } = Plot::new("fill line")
                .legend(Legend::default()) // with .name() method
                .view_aspect(4. / 3.)
                .include_x(0.) // show x axis label
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(points).fill(0.).name("fill"));
                    (plot_ui.pointer_coordinate(), plot_ui.plot_clicked())
                });

            let coordinate_text = if let Some(coordinate) = pointer_coordinate {
                format!("x: {:.02}, y: {:.02}", coordinate.x, coordinate.y)
            } else {
                "None".to_owned()
            };
            ui.label(&coordinate_text);
        }
        PlotPanel::Bars => {
            let bin_size = 5.;
            let bars = BarChart::new(
                (0..10)
                    .map(|x| {
                        if x == 5 {
                            Bar::new((x as f32 * bin_size) as f64, x as f64)
                                .width(bin_size as f64)
                                .fill(egui::color::Color32::LIGHT_RED)
                        } else {
                            Bar::new((x as f32 * bin_size) as f64, x as f64)
                                .width(bin_size as f64)
                        }
                    })
                    .collect(),
            )
            .color(egui::color::Color32::LIGHT_BLUE);

            Plot::new("my_barchart")
                .show(ui, |plot_ui| plot_ui.bar_chart(bars));
        }
        PlotPanel::BoxPlot => {
            let box1 = BoxPlot::new(vec![
                BoxElem::new(0.5, BoxSpread::new(1.5, 2.2, 2.5, 2.6, 3.1))
                    .name("Day 1"),
                BoxElem::new(2.5, BoxSpread::new(0.4, 1.0, 1.1, 1.4, 2.1))
                    .name("Day 2"),
                BoxElem::new(4.5, BoxSpread::new(1.7, 2.0, 2.2, 2.5, 2.9))
                    .name("Day 3"),
            ])
            .name("Experiment A");
            let box2 = BoxPlot::new(vec![BoxElem::new(
                1.0,
                BoxSpread::new(0.2, 0.5, 1.0, 2.0, 2.7),
            )
            .name("Day 2")])
            .name("Experiment B");
            Plot::new("Box Plot Demo").legend(Legend::default()).show(
                ui,
                |plot_ui| {
                    plot_ui.box_plot(box1);
                    plot_ui.box_plot(box2);
                },
            );
        }
    }
}
