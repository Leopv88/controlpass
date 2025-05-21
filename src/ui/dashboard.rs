use crate::db;
use egui::{Align, Layout, TextEdit, Ui};

pub struct Dashboard {
    connection_status: String,
    input_text: String,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            connection_status: "Не проверено".into(),
            input_text: String::new(),
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, config: &crate::config::AppConfig) {
        // Центр экрана
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.heading("Главное окно");
            use egui::Color32;
            use egui::RichText;

            let status_color = match self.connection_status.as_str() {
                "Успешно" => Color32::DARK_GREEN,
                "Не удалось" => Color32::RED,
                _ => Color32::GRAY,
            };

            ui.label(
                RichText::new(format!("Статус соединения: {}", self.connection_status))
                    .color(status_color)
                    .strong(),
            );

            if ui.button("Проверить соединение").clicked() {
                let status = db::test_connection(config.db_connection_string());
                self.connection_status = if status {
                    "Успешно".into()
                } else {
                    "Не удалось".into()
                };
            }

            ui.add_space(20.0);

            // Поле ввода по центру
            ui.label("Введите информацию:");
            ui.add(
                TextEdit::singleline(&mut self.input_text)
                    .hint_text("Ваш текст...")
                    .desired_width(300.0),
            );
        });

        // Кнопка "Выход" — вниз справа
        ui.with_layout(Layout::bottom_up(Align::Max), |ui| {
            ui.add_space(10.0);
            if ui.add_sized([200.0, 60.0], egui::Button::new("Выход")).clicked() {
                std::process::exit(0);
            }
        });
    }
}
