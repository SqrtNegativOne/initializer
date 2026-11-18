#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod model;
mod platform;
mod routines;
mod ui;

use app::Genesis;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let viewport = egui::ViewportBuilder::default()
        .with_title("Genesis")
        .with_fullscreen(true)
        .with_decorations(false)
        .with_transparent(true);

    let options = eframe::NativeOptions {
        viewport,
        renderer: eframe::Renderer::Glow,
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Genesis",
        options,
        Box::new(|cc| Ok(Box::new(Genesis::new(cc)))),
    )
}
