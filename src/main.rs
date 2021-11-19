#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;

fn main() {
    let app = app::Erabu::new();
    let window_options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::Vec2::new(400.0, 600.0)),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(Box::new(app), window_options);
}
