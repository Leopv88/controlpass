use crate::config::AppConfig;
use egui::{Window, Context};

#[derive(Clone)]
pub struct ConfigWindow {
    pub config: AppConfig,
    pub open: bool,
}

impl ConfigWindow {
    pub fn new(config: AppConfig) -> Self {
        Self { config, open: false }
    }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn ui(&mut self, ctx: &Context) {
        if self.open {
            Window::new("Настройка подключения")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("ODBC Connection String:");
                    ui.text_edit_singleline(&mut self.config.connection_string);
                    ui.horizontal(|ui| {
                        if ui.button("Сохранить").clicked() {
                            self.config.save();
                            self.open = false;
                        }
                        if ui.button("Отмена").clicked() {
                            self.open = false;
                        }
                    });
                });
        }
    }
}