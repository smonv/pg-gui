use egui::{popup, Button, CentralPanel, Id, TopBottomPanel, Window};

#[derive(Clone)]
struct App {
    pub win_open: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { win_open: true }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        TopBottomPanel::new(egui::panel::TopBottomSide::Top, "menu").show(ctx, |ui| {
            ui.menu_button("Main", |ui| {
                ui.menu_button("Sub Main", |ui| {
                    if ui.button("Close").clicked() {
                        ui.close_menu();
                    }
                })
            })
        });

        let mut win_open = true;

        if self.win_open {
            Window::new("Connection Panel")
                .open(&mut win_open)
                .show(ctx, |ui| {
                    ui.label("text");
                    if ui.button("Close").clicked() {
                        self.win_open = false
                    }
                });
        }
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
