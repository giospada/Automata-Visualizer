use crate::DisplayGraph::*;
use crate::Visualizer::*;
use crate::RegularExpression::*;
use crate::NFA::*;
use crate::DFA::*;
use eframe::egui;
use egui::{emath, Color32, Frame, Pos2, Rect, RichText, Window};

pub struct EguiApp {
    re: Visualizer,
    nfa: Visualizer,
    dfa: Visualizer,
    error: Option<String>,
    regex_text: String,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            re: Visualizer::new("Regex Syntax Tree".to_string()),
            nfa: Visualizer::new("NFA".to_string()),
            dfa: Visualizer::new("DFA".to_string()),
            error: None,
            regex_text: String::new(),
        }
    }
}

impl EguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Main").show(ctx, |ui| {
            for (index, visualizer) in [&mut self.re, &mut self.nfa, &mut self.dfa].into_iter().enumerate() {
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

                            // TODO: questa parte è bruttissima, hardcodato il match per le istruzioni di creazione del visualizer
                            // bisogna fare un mapper tipo: create_visualizer_from_regex(re) -> Visualizer

                            let graph = if index == 0 {
                                re.into()
                            } else if index == 1 {
                                NFA::from_regex(&re).into()
                            } else {
                                let nfa = NFA::from_regex(&re);
                                DFA::from_nfa(&nfa).into()
                            };
                            visualizer.generate_graph(graph);
                            self.error=None;
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
        for visualizer in [&mut self.nfa,&mut self.dfa, &mut self.re].into_iter() {
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
