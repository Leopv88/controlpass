pub mod config_window;
pub mod dashboard;

use crate::config::AppConfig;
use egui::{Context, TopBottomPanel, CentralPanel};

pub struct App {
    dashboard: dashboard::Dashboard,
    config_window: config_window::ConfigWindow,
}

impl App {
    pub fn new() -> Self {
        let app_config = AppConfig::load();
        Self {
            dashboard: dashboard::Dashboard::new(),
            config_window: config_window::ConfigWindow::new(app_config),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("ControlPass");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Настройка").clicked() {
                        self.config_window.open();
                    }
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            self.dashboard.ui(ui, &self.config_window.config);
        });

        self.config_window.ui(ctx);
    }
}