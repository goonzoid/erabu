use eframe::egui;
use eframe::egui::{Align, Button, Color32, Label, Layout, ScrollArea, TextEdit, Ui, Window};
use eframe::epi;
use serde::{Deserialize, Serialize};

const PADDING: f32 = 8.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

#[derive(Serialize, Deserialize, Default)]
pub struct Erabu {
    projects: Vec<Project>,

    #[serde(skip)]
    ui_state: UIState,
}

#[derive(Serialize, Deserialize)]
struct Project {
    title: String,
    tags: Vec<String>,
}

#[derive(Default)]
struct UIState {
    filter: String,
    adding_project: bool,
    deleted_project_title: String,
    project_template: ProjectTemplate,
    title_field_needs_focus: bool,
    party_time: bool,
    random_number: Option<usize>,
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

impl epi::App for Erabu {
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
        self.update_data();

        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            render_controls(&mut self.ui_state, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // this actually creates a very small gap at the top for release
            // builds, but let's not worry about that for now!
            ui.vertical_centered(|ui| {
                egui::warn_if_debug_build(ui);
            });

            render_project_list(&self.projects, &mut self.ui_state, ui);
        });

        Window::new("new project")
            .open(&mut self.ui_state.adding_project)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                let response = render_add_project_form(&mut self.ui_state.project_template, ui);
                if self.ui_state.title_field_needs_focus {
                    response.request_focus();
                    self.ui_state.title_field_needs_focus = false;
                }
            });

        Window::new("you should...")
            .open(&mut self.ui_state.party_time)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| match self.ui_state.random_number {
                Some(r) => {
                    let projects = filter_projects(self.ui_state.filter.as_str(), &self.projects);
                    let project = projects[r % projects.len()];
                    ui.with_layout(Layout::top_down(Align::Center), |ui| {
                        ui.add(Label::new(&project.title).text_color(WHITE).heading());
                    });
                }
                None => {
                    self.ui_state.random_number = rand::random();
                }
            });
    }
}

impl Erabu {
    fn update_data(&mut self) {
        if !self.ui_state.deleted_project_title.is_empty() {
            self.projects
                .retain(|p| p.title != self.ui_state.deleted_project_title);
        }

        if self.ui_state.project_template.completed {
            self.ui_state.adding_project = false;
            self.projects.push(Project {
                title: self.ui_state.project_template.title.trim().to_owned(),
                tags: self
                    .ui_state
                    .project_template
                    .tags
                    .split_whitespace()
                    .map(|x| x.to_owned())
                    .collect(),
            });
        }

        if !self.ui_state.adding_project {
            self.ui_state.project_template.clear();
            self.ui_state.title_field_needs_focus = true;
        }

        if !self.ui_state.party_time {
            self.ui_state.random_number = None;
        }
    }
}

fn render_project_list(projects: &[Project], ui_state: &mut UIState, ui: &mut Ui) {
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            for project in filter_projects(ui_state.filter.as_str(), projects) {
                ui.horizontal(|ui| {
                    ui.add(Label::new(&project.title).text_color(WHITE).heading());
                    ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                        let response = ui.add(Button::new("❌").frame(false));
                        if response.clicked() {
                            ui_state.deleted_project_title = project.title.clone();
                        }
                    });
                });
                ui.horizontal(|ui| {
                    for tag in &project.tags {
                        ui.label(tag);
                    }
                });
                ui.add_space(PADDING);
            }
        });
}

fn render_controls(ui_state: &mut UIState, ui: &mut Ui) {
    ui.add_space(PADDING);
    ui.horizontal(|ui| {
        ui.label("search:");
        ui.add(TextEdit::singleline(&mut ui_state.filter).desired_width(f32::INFINITY));
    });
    ui.add_space(PADDING);
    ui.add_space(PADDING);
    ui.horizontal(|ui| {
        if ui.button("➕").clicked() {
            ui_state.adding_project = true;
        }
        ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
            if ui.button("▶").clicked() {
                ui_state.random_number = rand::random();
                ui_state.party_time = true;
            }
        });
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
