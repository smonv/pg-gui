use egui::{
    panel::{Side, TopBottomSide},
    popup, Button, CentralPanel, Id, SidePanel, TopBottomPanel, Window,
};
use egui_extras::{Size, TableBuilder};

#[derive(Clone)]
struct App {}

impl Default for App {
    fn default() -> Self {
        App {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        TopBottomPanel::new(TopBottomSide::Top, "menu").show(ctx, |ui| {
            ui.menu_button("Main", |ui| {
                ui.menu_button("Sub Main", |ui| {
                    if ui.button("Close").clicked() {
                        ui.close_menu();
                    }
                })
            })
        });
        SidePanel::new(Side::Left, "side").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add(Button::new("Conn 1"));
                ui.add(Button::new("Conn 2"));
                ui.add(Button::new("Conn 3"));
                ui.add(Button::new("+"));
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            TopBottomPanel::new(TopBottomSide::Top, "main-menu").show(ui.ctx(), |ui| {
                ui.horizontal_wrapped(|ui| {
                    let mut checked = false;
                    ui.checkbox(&mut checked, "Field enable");
                    ui.label("Field");
                    ui.label("Input")
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label("Field");
                    ui.label("Input")
                });
            });

            CentralPanel::default().show(ui.ctx(), |ui| {
                TableBuilder::new(ui)
                    .column(Size::remainder())
                    .column(Size::remainder())
                    .header(30., |mut header| {
                        header.col(|ui| {
                            ui.heading("id");
                        });
                        header.col(|ui| {
                            ui.heading("name");
                        });
                    })
                    .body(|_| {});
            });
        });
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "PG GUI",
        native_options,
        Box::new(|_cc| Box::new(App::default())),
    )
}
