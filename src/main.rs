#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use eframe::{egui, epi, run_native, NativeOptions};

struct Erabu;

impl epi::App for Erabu {
    fn name(&self) -> &str {
        "erabu"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.name());
            egui::warn_if_debug_build(ui);
        });
    }
}

fn main() {
    let app = Erabu;
    let window_options = NativeOptions::default();
    run_native(Box::new(app), window_options);
}
