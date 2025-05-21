mod ui;
mod db;
mod config;
mod license;

use eframe::{NativeOptions, run_native};
use egui::ViewportBuilder;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_fullscreen(true)
            .with_decorations(false),
        ..Default::default()
    };
    
    let app = ui::App::new();
    run_native("ControlPass", options, Box::new(|_cc| Box::new(app)))
}