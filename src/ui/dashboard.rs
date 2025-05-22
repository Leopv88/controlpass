//главное окно
use crate::db; //Подключает модуль db, содержащий функции работы с базой данных.
use egui::{Align, Layout, TextEdit, Ui};
/*
Импортируются структуры и компоненты из библиотеки egui, необходимые для отрисовки интерфейса:
Align — выравнивание (Center, Max, и т.д.);
Layout — определяет расположение элементов (top_down, bottom_up);
TextEdit — однострочное текстовое поле;
Ui — объект, через который строится интерфейс в egui.
*/


pub struct Dashboard { //Создаётся структура
    connection_status: String, //текущий статус соединения с БД (например, "Успешно" или "Не удалось").
    input_text: String, //содержимое текстового поля ввода, которое пользователь вводит вручную.
}

impl Dashboard {
    pub fn new() -> Self { //Инициализирует Dashboard с начальными значениями
        Self {
            connection_status: "Не проверено".into(), //строка "Не проверено" — до первой попытки соединения.
            input_text: String::new(), //пустая строка
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, config: &crate::config::AppConfig) {
        // Центр экрана
        ui.with_layout(Layout::top_down(Align::Center), |ui| { //Создаёт вертикальный макет (top_down — сверху вниз) и центрирует его по горизонтали.
            ui.heading("Главное окно"); //Выводит заголовок окна
            use egui::Color32; //Color32: тип для цвета (включая RGB).
            use egui::RichText;//позволяет стилизовать текст (цвет, жирность и т.п.).

            let status_color = match self.connection_status.as_str() { //Устанавливает цвет текста в зависимости от статуса соединения:
                "Успешно" => Color32::DARK_GREEN,
                "Не удалось" => Color32::RED,
                _ => Color32::GRAY, //если что-то иное
            };

            ui.label( //Выводит текст с цветом и жирным шрифтом: текущий статус соединения
                RichText::new(format!("Статус соединения: {}", self.connection_status))
                    .color(status_color)
                    .strong(),
            );

            if ui.button("Проверить соединение").clicked() { //Создаёт кнопку. Если нажали — вызывается
                let status = db::test_connection(config.db_connection_string()); //Проверяет соединение с базой данных с помощью строки подключения из AppConfig
                self.connection_status = if status {
                    "Успешно".into()
                } else {
                    "Не удалось".into()
                };
            }

            ui.add_space(20.0); //Добавляет немного отступа между кнопкой и полем ввода

            // Поле ввода по центру
            ui.label("Введите информацию:");
            ui.add(
                TextEdit::singleline(&mut self.input_text)
                    .hint_text("Ваш текст...")
                    .desired_width(300.0),
            );
            /*
            Однострочное поле для ввода.
            Текст сохраняется в переменной input_text.
            Отображается подсказка ("Ваш текст...").
            Ширина поля — 300 пикселей.
            */
        });

        // Кнопка "Выход" — вниз справа
        ui.with_layout(Layout::bottom_up(Align::Max), |ui| { //Создаётся макет снизу вверх, выровненный по правому краю (Max).
            ui.add_space(10.0);//Добавляет немного отступа.
            if ui.add_sized([200.0, 60.0], egui::Button::new("Выход")).clicked() { //Создаёт большую кнопку Выход (размером 200x60 пикселей).
                std::process::exit(0);
            }
        });
    }
}
