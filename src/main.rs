#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use eframe::{egui, egui::Ui, epi, run_native, NativeOptions};

const PADDING: f32 = 8.0;
const WHITE: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);

struct Erabu {
    projects: Vec<Project>,
}

impl Erabu {
    fn new() -> Erabu {
        let iter = (0..99).map(|i| Project {
            title: format!("a kinda long project title example {}", i),
            tags: vec!["an tag", "an other tag"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        });
        Erabu {
            projects: Vec::from_iter(iter),
        }
    }

    fn render_project_list(&self, ui: &mut Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for p in &self.projects {
                    self.render_project(ui, p);
                }
            });
    }

    fn render_project(&self, ui: &mut Ui, project: &Project) {
        ui.add(egui::Label::new(&project.title).text_color(WHITE).heading());
        ui.horizontal(|ui| {
            for tag in &project.tags {
                ui.label(tag);
            }
        });
        ui.add_space(PADDING);
    }
}

struct Project {
    title: String,
    tags: Vec<String>,
}

impl epi::App for Erabu {
    fn name(&self) -> &str {
        "erabu"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // this actually creates a very small gap at the top for release
            // builds, but let's not worry about that for now!
            ui.vertical_centered(|ui| {
                egui::warn_if_debug_build(ui);
            });

            self.render_project_list(ui);
        });
    }
}

fn main() {
    let app = Erabu::new();
    let mut window_options = NativeOptions::default();
    let size_x = 400.0;
    let size_y = 600.0;
    window_options.initial_window_size = Some(egui::Vec2::new(size_x, size_y));
    run_native(Box::new(app), window_options);
}
