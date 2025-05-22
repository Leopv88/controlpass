pub mod config_window;
pub mod dashboard;
//модули  доступны в других частях программы как ui::dashboard и ui::config_window

use crate::config::AppConfig; //подключает структуру конфигурации, которая хранит настройки (например, строку подключения).

use egui::{Context, TopBottomPanel, CentralPanel};
/*
use egui::{...} — импортирует элементы интерфейса из библиотеки egui:
Context — контекст отрисовки UI.
TopBottomPanel — верхняя или нижняя панель в интерфейсе.
CentralPanel — основная центральная часть окна (где размещаются элементы управления).
*/

pub struct App { //создаётся основная структура App, которая представляет состояние интерфейса. Состоит из двух окон.
    dashboard: dashboard::Dashboard,
    config_window: config_window::ConfigWindow,
}

impl App { //создание App
    pub fn new() -> Self {
        let app_config = AppConfig::load(); //Загружает конфигурацию из файла (например, config.json). Это нужно, чтобы заполнить поля в окне настроек или сразу протестировать соединение с БД.
        Self {
            dashboard: dashboard::Dashboard::new(), // создание главного окна
            config_window: config_window::ConfigWindow::new(app_config), //Создаёт окно настроек, передавая туда загруженную конфигурацию
        }
    }
}// Конструктор возвращает готовое состояние приложения, готовое к отрисовке

impl eframe::App for App { //Здесь реализуется трейт App из библиотеки eframe. Этот трейт говорит фреймворку, как отрисовывать интерфейс при каждом кадре
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        /*
        ctx: &Context — контекст рисования, передаётся каждый кадр.
        _frame: &mut eframe::Frame — информация о фрейме (например, его закрытие, перерисовка и т.д.). Здесь не используется, поэтому с подчёркиванием.
        */
        TopBottomPanel::top("top_panel").show(ctx, |ui| { //Создаётся верхняя панель с названием
            ui.horizontal(|ui| { //размещение элементов в строку
                ui.label("ControlPass"); //заголовок программы
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { //кнопка "Настройка" выравнивается вправо
                    if ui.button("Настройка").clicked() { //При нажатии на неё вызывается self.config_window.open(), чтобы открыть окно настроек. 
                        self.config_window.open(); 
                    }
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| { //Центральная панель
            self.dashboard.ui(ui, &self.config_window.config); 
            /*
            Вызывается self.dashboard.ui(...) — отрисовка главного окна
            В аргументы передаётся текущая конфигурация подключения (&self.config_window.config), чтобы можно было, например, протестировать соединение.
            */
        });

        self.config_window.ui(ctx);
        /*
        Это отрисовка всплывающего окна с конфигурацией, если оно открыто.
        config_window сам управляет своим состоянием (открыт/закрыт, поля, кнопки).
    📌 Этот вызов нужен, чтобы окно появлялось поверх остального интерфейса при необходимости.
        */
    }
}