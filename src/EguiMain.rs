use eframe::egui;
use egui::{emath, Color32, Frame, Pos2, Rect, RichText, Window};

use crate::DisplayGraph::{DisplayGraph, DisplayGraphParameter};
use crate::Visualizer::Visualizer;
use crate::RegularExpression::ReOperator;
use crate::NFA::NFA;
use crate::DFA::DFA;

pub struct EguiApp  {
    error: Option<String>,
    regex_text: String,

    // This is indexed accordingly
    // 0: Regex
    // 1: NFA
    // 2: DFA
    // 3: Minimized DFA
    // a union structure would be useful for accessing the Visualizers
    // with both indixes and names, but it's problematic how to do it
    // in rust.
    to_visualize: [Visualizer; 4],

}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            error: None,
            regex_text: String::new(),

            to_visualize: [
                Visualizer::new("Regex Syntax Tree".to_string()), 
                Visualizer::new("NFA".to_string()),
                Visualizer::new("DFA".to_string()),
                Visualizer::new("Minimized DFA".to_string()),

            ],
        }
    }
}

impl EguiApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }

    pub fn get_converter(index: i32) -> impl Fn(ReOperator) -> DisplayGraph {
        match index {
            0 => |re: ReOperator| re.into(),
            1 => |re: ReOperator| NFA::from(&re).into(),
            2 => |re: ReOperator| DFA::from(&NFA::from(&re)).into(),
            3 => |re: ReOperator| DFA::from(&NFA::from(&re)).get_minimized_dfa().into(),
            _ => panic!("Invalid index"),
        }
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Main").show(ctx, |ui| {
            for (index, visualizer) in self.to_visualize.iter_mut().enumerate() {
                ui.heading(&visualizer.box_title);
                if index == 0 {
                    ui.horizontal(|ui| {
                        ui.label("inserisci la regex");
                        ui.text_edit_singleline(&mut self.regex_text)
                            .on_hover_text("Enter a regular expression");
                    });
                }
                if ui.button(format!("Generate {}", visualizer.box_title)).clicked() {
                    match ReOperator::from_string(&self.regex_text) {
                        Ok(re) => {
                            visualizer.set_graph(Self::get_converter(index as i32)(re));
                            self.error = None;
                        }
                        
                        Err(e) => {
                            self.error = Some(e.to_string());
                        }
                    };
                }

                ui.collapsing(format!("{} visualizer option", visualizer.box_title), |ui| {
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
            let syntaxTree = Window::new(format!("{}", visualizer.box_title));
            let syntaxTree = syntaxTree.open(&mut visualizer.is_win_open);
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
