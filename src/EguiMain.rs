use crate::DisplayGraph::*;
use crate::Visualizer::*;
use crate::RegularExpression::*;
use crate::NFA::*;
use crate::DFA::*;
use eframe::egui;
use egui::{emath, Color32, Frame, Pos2, Rect, RichText, Window};

pub struct EguiApp  {
    error: Option<String>,
    regex_text: String,

    // This is indexed accordingly
    // 0: Regex
    // 1: NFA
    // 2: DFA
    to_visualize: [Visualizer; 3],
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            error: None,
            regex_text: String::new(),

            to_visualize: [
                Visualizer::new(ReOperator::get_name()), 
                Visualizer::new(NFA::get_name()),
                Visualizer::new(DFA::get_name()),
            ],
        }
    }
}

impl EguiApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }

    pub fn get_converter(index: i32) -> impl Fn(ReOperator) -> Visualizer {
        match index {
            0 => |re| Visualizer::from(re),
            1 => |re| Visualizer::from(NFA::from(&re)),
            2 => |re| Visualizer::from(DFA::from(&NFA::from(&re))),
            _ => panic!("Invalid index"),
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Main").show(ctx, |ui| {
            for (index, visualizer) in self.to_visualize.iter_mut().enumerate() {
                ui.heading(&visualizer.name);
                if index == 0 {
                    ui.horizontal(|ui| {
                        ui.label("inserisci la regex");
                        ui.text_edit_singleline(&mut self.regex_text)
                            .on_hover_text("Enter a regular expression");
                    });
                }
                if ui.button(format!("Generate {}", visualizer.name)).clicked() {
                    match ReOperator::from_string(&self.regex_text) {
                        Ok(re) => {
                            *visualizer = Self::get_converter(index as i32)(re);
                            self.error = None;
                        }
                        
                        Err(e) => {
                            self.error = Some(e.to_string());
                        }
                    };
                }

                ui.collapsing(format!("{} visualizer option", visualizer.name), |ui| {
                    ui.add(
                        egui::Slider::new(&mut visualizer.padding_x, 10.0..=100.0)
                            .text("padding x"),
                    );
                    ui.add(
                        egui::Slider::new(&mut visualizer.padding_y, 10.0..=100.0)
                            .text("padding y"),
                    );
                    ui.add(
                        egui::Slider::new(&mut visualizer.size_node, 10.0..=100.0)
                            .text("node size"),
                    );
                });
            }
            if let Some(err) = &self.error {
                ui.label(RichText::new(err).color(Color32::RED));
            }
          
        });
        for visualizer in self.to_visualize.iter_mut() {
            visualizer.check_open();
            let syntaxTree = Window::new(format!("{}", visualizer.name));
            let syntaxTree = syntaxTree.open(&mut visualizer.open);
            let syntaxTree = syntaxTree.scroll2([true, true]);
            syntaxTree.show(ctx, |ui| {
                Frame::canvas(ui.style()).show(ui, |ui| {
                    if let Some(tree) = &mut visualizer.graph {
                        let scren_size = tree.position(DisplayGraphParameter {
                            padding_x: visualizer.padding_x,
                            padding_y: visualizer.padding_y,
                            node_size: visualizer.size_node,
                        });
                        let (mut response, painter) =
                            ui.allocate_painter(scren_size, egui::Sense::hover());

                        let to_screen = emath::RectTransform::from_to(
                            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                            response.rect,
                        );
                        tree.drag_nodes(to_screen, ui, &mut response);
                        tree.draw(&painter, to_screen, &ui);
                    }
                })
            });
            
        }
    }
}
