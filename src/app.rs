use eframe::egui;
use eframe::egui::{
    Align, Color32, Label, Layout, Pos2, ScrollArea, Sense, Stroke, TextEdit, Ui, Vec2, Window,
};
use eframe::epi;

const PADDING: f32 = 8.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

pub struct Erabu {
    projects: Vec<Project>,
    filter: String,
    adding_project: bool,
    new_project_title: String,
    new_project_tags: String,
    project_ready_to_add: bool,
    deleted_project_title: String,
}

struct Project {
    title: String,
    tags: Vec<String>,
}

impl Default for Erabu {
    fn default() -> Self {
        let iter = (0..99).map(|i| Project {
            title: format!("a kinda long project title example {}", i),
            tags: vec!["an tag", "an other tag"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        });
        Self {
            projects: Vec::from_iter(iter),
            filter: "".to_owned(),
            adding_project: false,
            new_project_title: "".to_owned(),
            new_project_tags: "".to_owned(),
            project_ready_to_add: false,
            deleted_project_title: "".to_owned(),
        }
    }
}

impl epi::App for Erabu {
    fn name(&self) -> &str {
        "erabu"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        self.update_data();

        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            self.render_controls(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // this actually creates a very small gap at the top for release
            // builds, but let's not worry about that for now!
            ui.vertical_centered(|ui| {
                egui::warn_if_debug_build(ui);
            });

            self.render_project_list(ui);
        });

        Window::new("new project")
            .open(&mut self.adding_project)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(PADDING);
                ui.label("title:");
                ui.add(
                    TextEdit::singleline(&mut self.new_project_title).desired_width(f32::INFINITY),
                );
                ui.label("tags:");
                ui.add(
                    TextEdit::singleline(&mut self.new_project_tags).desired_width(f32::INFINITY),
                );
                ui.add_space(PADDING);
                ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                    if ui.button("add").clicked() {
                        self.project_ready_to_add = true;
                    }
                });
            });
    }
}

impl Erabu {
    fn update_data(&mut self) {
        if self.deleted_project_title != "" {
            self.projects
                .retain(|p| p.title != self.deleted_project_title);
        }

        if self.project_ready_to_add {
            self.adding_project = false;
            self.projects.push(Project {
                title: self.new_project_title.trim().to_owned(),
                tags: self
                    .new_project_tags
                    .split_whitespace()
                    .map(|x| x.to_owned())
                    .collect(),
            });
            self.project_ready_to_add = false;
        }

        if !self.adding_project {
            self.new_project_title.clear();
            self.new_project_tags.clear();
        }
    }

    fn render_project_list(&mut self, ui: &mut Ui) {
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for project in filter_projects(self.filter.as_str(), &self.projects) {
                    ui.add(Label::new(&project.title).text_color(WHITE).heading());
                    ui.horizontal(|ui| {
                        for tag in &project.tags {
                            ui.label(tag);
                        }
                        ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                            if ui.button("delete").clicked() {
                                self.deleted_project_title = project.title.clone();
                            }
                        });
                    });
                    ui.add_space(PADDING);
                }
            });
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.add(TextEdit::singleline(&mut self.filter).desired_width(f32::INFINITY));
        ui.add_space(PADDING);
        ui.horizontal(|ui| {
            if ui.button("add project").clicked() {
                self.adding_project = true;
            }
            ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                self.draw_play_icon(ui);
            });
        });
        ui.add_space(PADDING);
    }

    fn draw_play_icon(&self, ui: &mut Ui) {
        let size = Vec2 { x: 28.0, y: 18.0 };
        let stroke = Stroke::new(2.0, WHITE);
        let (response, painter) = ui.allocate_painter(size, Sense::click());
        let rect = response.rect;

        // use the height as the width to give us a square drawing area with some
        // padding on the right - yes, it's a dirty hack
        let (c, h, w) = (rect.center(), rect.height(), rect.height());
        let top_left = Pos2 {
            x: c.x - w / 2.0 + 2.0,
            y: c.y - h / 2.0,
        };
        let bottom_left = Pos2 {
            x: c.x - w / 2.0 + 2.0,
            y: c.y + h / 2.0,
        };
        let right_above = Pos2 {
            x: c.x + w / 2.0 - 2.0,
            y: c.y - 0.5,
        };
        let right_below = Pos2 {
            x: c.x + w / 2.0 - 2.0,
            y: c.y + 0.5,
        };

        painter.line_segment([top_left, bottom_left], stroke);
        painter.line_segment([top_left, right_below], stroke);
        painter.line_segment([bottom_left, right_above], stroke);
    }
}

fn filter_projects<'a>(filter: &'a str, projects: &'a Vec<Project>) -> Vec<&'a Project> {
    return projects
        .iter()
        .filter(|project| {
            project.title.contains(filter) || project.tags.iter().any(|tag| tag.contains(filter))
        })
        .collect();
}
