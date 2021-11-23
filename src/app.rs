use eframe::egui;
use eframe::egui::{Align, Button, Color32, Label, Layout, ScrollArea, TextEdit, Ui, Window};
use eframe::epi;
use serde::{Deserialize, Serialize};

const PADDING: f32 = 8.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

#[derive(Serialize, Deserialize, Default)]
pub struct ErabuApp {
    projects: Vec<Project>,

    #[serde(skip)]
    controller: Controller,
}

#[derive(Serialize, Deserialize)]
struct Project {
    title: String,
    tags: Vec<String>,
}

#[derive(Default)]
struct Controller {
    filter: String,
    adding_project: bool,
    deleted_project_title: String,
    project_template: ProjectTemplate,
    title_field_needs_focus: bool,
    party_time: bool,
    random_number: Option<usize>,
}

impl Controller {
    fn update(&mut self, projects: &mut Vec<Project>) {
        self.remove_deleted_project(projects);
        self.add_project(projects);
        self.clear_random_number();
    }

    fn remove_deleted_project(&mut self, projects: &mut Vec<Project>) {
        if !self.deleted_project_title.is_empty() {
            projects.retain(|p| p.title != self.deleted_project_title);
            self.deleted_project_title.clear();
        }
    }

    fn add_project(&mut self, projects: &mut Vec<Project>) {
        if self.project_template.completed {
            self.adding_project = false;
            projects.push(Project {
                title: self.project_template.title.trim().to_owned(),
                tags: self
                    .project_template
                    .tags
                    .split_whitespace()
                    .map(|x| x.to_owned())
                    .collect(),
            });
        }

        if !self.adding_project {
            self.project_template.clear();
            self.title_field_needs_focus = true;
        }
    }

    fn clear_random_number(&mut self) {
        if !self.party_time {
            self.random_number = None;
        }
    }
}

#[derive(Default)]
struct ProjectTemplate {
    title: String,
    tags: String,
    completed: bool,
}

impl ProjectTemplate {
    fn clear(&mut self) {
        self.title.clear();
        self.tags.clear();
        self.completed = false;
    }
}

impl epi::App for ErabuApp {
    fn name(&self) -> &str {
        "erabu"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    // Saving on exit isn't reliable right now (see https://github.com/emilk/egui/issues/597
    // and https://github.com/emilk/egui/issues/814) so let's auto save often.
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(1)
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        self.controller.update(&mut self.projects);

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            render_menu_bar(&mut self.controller, ui);
        });

        egui::TopBottomPanel::bottom("search_and_play").show(ctx, |ui| {
            render_search_and_play(&mut self.controller, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            render_project_list(&self.projects, &mut self.controller, ui);
        });

        Window::new("new project")
            .open(&mut self.controller.adding_project)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                let response = render_add_project_form(&mut self.controller.project_template, ui);
                if self.controller.title_field_needs_focus {
                    response.request_focus();
                    self.controller.title_field_needs_focus = false;
                }
            });

        Window::new("⏩")
            .open(&mut self.controller.party_time)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| match self.controller.random_number {
                Some(r) => {
                    let projects = filter_projects(self.controller.filter.as_str(), &self.projects);
                    let project = projects[r % projects.len()];
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.add(Label::new(&project.title).text_color(WHITE).heading());
                    });
                }
                None => {
                    self.controller.random_number = rand::random();
                }
            });
    }
}

fn render_project_list(projects: &[Project], controller: &mut Controller, ui: &mut Ui) {
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            for project in filter_projects(controller.filter.as_str(), projects) {
                ui.horizontal(|ui| {
                    ui.add(Label::new(&project.title).text_color(WHITE).heading());
                    ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                        let response = ui.add(Button::new("❌").frame(false));
                        if response.clicked() {
                            controller.deleted_project_title = project.title.clone();
                        }
                    });
                });
                ui.horizontal(|ui| {
                    for tag in &project.tags {
                        ui.label(tag);
                    }
                    if project.tags.is_empty() {
                        ui.label(""); // blank label for spacing
                    }
                });
                ui.separator();
            }
        });
}

fn render_menu_bar(controller: &mut Controller, ui: &mut Ui) {
    ui.add_space(PADDING);
    ui.horizontal(|ui| {
        if ui.button("➕").clicked() {
            controller.adding_project = true;
        }

        ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
            egui::warn_if_debug_build(ui);
        });
    });
    ui.add_space(PADDING);
}

fn render_search_and_play(controller: &mut Controller, ui: &mut Ui) {
    ui.add_space(PADDING);
    ui.horizontal(|ui| {
        ui.label("search:");
        ui.add(TextEdit::singleline(&mut controller.filter).desired_width(f32::INFINITY));
    });
    ui.add_space(PADDING);
    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        if ui.button("▶").clicked() {
            controller.random_number = rand::random();
            controller.party_time = true;
        }
    });
    ui.add_space(PADDING);
}

fn render_add_project_form(project_template: &mut ProjectTemplate, ui: &mut Ui) -> egui::Response {
    ui.add_space(PADDING);
    ui.label("title:");
    let title_field =
        ui.add(TextEdit::singleline(&mut project_template.title).desired_width(f32::INFINITY));
    ui.label("tags:");
    ui.add(TextEdit::singleline(&mut project_template.tags).desired_width(f32::INFINITY));
    ui.add_space(PADDING);
    ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
        if ui.button("add").clicked() {
            project_template.completed = true;
        }
    });
    title_field
}

fn filter_projects<'a>(filter: &'a str, projects: &'a [Project]) -> Vec<&'a Project> {
    return projects
        .iter()
        .filter(|project| {
            project.title.contains(filter) || project.tags.iter().any(|tag| tag.contains(filter))
        })
        .collect();
}
