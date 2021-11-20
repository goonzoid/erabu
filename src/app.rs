use eframe::{egui, epi};

const PADDING: f32 = 8.0;
const WHITE: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);

pub struct Erabu {
    projects: Vec<Project>,
    filter: String,
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
    }
}

impl Erabu {
    fn update_data(&mut self) {
        if self.deleted_project_title != "" {
            self.projects
                .retain(|p| p.title != self.deleted_project_title);
        }
    }

    fn render_project_list(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for project in filter_projects(self.filter.as_str(), &self.projects) {
                    ui.add(egui::Label::new(&project.title).text_color(WHITE).heading());
                    ui.horizontal(|ui| {
                        for tag in &project.tags {
                            ui.label(tag);
                        }
                        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                            if ui.button("delete").clicked() {
                                self.deleted_project_title = project.title.clone();
                            }
                        });
                    });
                    ui.add_space(PADDING);
                }
            });
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.add_space(PADDING);
        ui.add(egui::TextEdit::singleline(&mut self.filter).desired_width(f32::INFINITY));
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| {
            self.draw_play_icon(ui);
        });
        ui.add_space(PADDING);
    }

    fn draw_play_icon(&self, ui: &mut egui::Ui) {
        let size = egui::Vec2::splat(18.0);
        let stroke = egui::Stroke::new(2.0, WHITE);
        let (response, painter) = ui.allocate_painter(size, egui::Sense::click());
        let rect = response.rect;

        let (c, h, w) = (rect.center(), rect.height(), rect.width());
        let top_left = egui::Pos2 {
            x: c.x - w / 2.0 + 2.0,
            y: c.y - h / 2.0,
        };
        let bottom_left = egui::Pos2 {
            x: c.x - w / 2.0 + 2.0,
            y: c.y + h / 2.0,
        };
        let right_above = egui::Pos2 {
            x: c.x + w / 2.0 - 2.0,
            y: c.y - 0.5,
        };
        let right_below = egui::Pos2 {
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
