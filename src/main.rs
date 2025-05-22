mod ui; //подключение модулей с именем mod с папках или файлов
mod db;
mod config;
mod license;

use eframe::{NativeOptions, run_native};
use egui::ViewportBuilder;
/*
NativeOptions — настройки окна, передаваемые при запуске приложения
run_native — запуск нативного десктопного приложения (через eframe)
ViewportBuilder — позволяет настраивать внешний вид окна (например, полноэкранный режим, убрать рамки)
*/

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default() //создаём "строитель" для окна (builder).
            .with_fullscreen(true) //запускаем приложение в полноэкранном режиме.
            .with_decorations(false), //убираем системные рамки, заголовок и кнопки окна.
        ..Default::default()
    };
    
    let app = ui::App::new(); //создаётся экземпляр пользовательского интерфейса
    run_native("ControlPass", options, Box::new(|_cc| Box::new(app)))
    /*
    "ControlPass" — имя окна (будет отображаться в заголовке, если не отключены рамки).
    options — передаём параметры окна (NativeOptions, выше).
    Box::new(|_cc| Box::new(app)) — замыкание (анонимная функция), возвращающая созданное приложение (App), обёрнутое в Box.
    */
}