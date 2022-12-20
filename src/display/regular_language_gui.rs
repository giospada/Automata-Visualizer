use crate::automata::{NfaStates, ReOperator, DFA, NFA};
use crate::display::DisplayGraph;
use crate::utils::IntoGraph;
use egui::{Context, Ui};

use std::collections::VecDeque;

use crate::display::Visualizer;

pub struct RegularGrammarObjects {
    regular_expression: Option<ReOperator>,
    nfa: Option<NFA>,
    dfa: Option<DFA<NfaStates>>,
    min_dfa: Option<DFA<NfaStates>>,
}

impl RegularGrammarObjects {
    fn new() -> Self {
        Self {
            regular_expression: None,
            nfa: None,
            dfa: None,
            min_dfa: None,
        }
    }
}

pub struct RegularLanguageGui {
    object_copy: RegularGrammarObjects,
    error_log: VecDeque<String>,
    visualizers: [Visualizer; 4],
}

const MAX_LOG_QUEUE: usize = 5;

impl RegularLanguageGui {
    fn get_regular_expression_visualizer() -> Visualizer {
        Visualizer::from_title_and_fromlist(
            "Regular Expression",
            vec![|ui, reg, input, log| {
                ui.text_edit_singleline(input);
                if ui.button("Regular Expression").clicked() {
                    match ReOperator::from_string(input) {
                        Ok(elem) => {
                            let display_graph = DisplayGraph::from(elem.into_graph());
                            reg.regular_expression = Some(elem);
                            return Some(display_graph);
                        }
                        Err(_) => {
                            log.push_back(String::from("Impossibile creare la regular expression"));
                        }
                    };
                }
                None
            }],
        )
    }
    fn get_NFA_visualizer() -> Visualizer {
        Visualizer::from_title_and_fromlist(
            "NFA",
            vec![
                |ui, reg, input, log| {
                    ui.text_edit_multiline(input);
                    if ui.button("From Input Text").clicked() {
                        match serde_json::from_str::<NFA>(input.as_str()) {
                            Ok(elem) => {
                                let display_graph = DisplayGraph::from(elem.into_graph());
                                reg.nfa = Some(elem);
                                return Some(display_graph);
                            }
                            Err(e) => {
                                log.push_back(e.to_string());
                            }
                        };
                    }
                    if ui.button("Set Input Text As The Current NFA").clicked() {
                        match reg.nfa {
                            Some(ref nfa) => {
                                match serde_json::to_string(nfa) {
                                    Ok(fr) => {
                                        *input = String::from(fr);
                                    }
                                    Err(e) => {
                                        log.push_back(e.to_string());
                                    }
                                };
                            }
                            None => {
                                log.push_back(String::from("Nessun NFA è Settato"));
                            }
                        }
                    }
                    None
                },
                |ui, reg, _, log| {
                    if ui.button("From Regular Expression").clicked() {
                        match &reg.regular_expression {
                            Some(elem) => {
                                let elem=NFA::from(elem);
                                let display_graph = DisplayGraph::from(elem.into_graph());
                                reg.nfa = Some(elem);
                                return Some(display_graph);
                            }
                            None => {
                                log.push_back(String::from(
                                    "Try to visualize a regular Expression First",
                                ));
                            }
                        };
                    }
                    None
                },
            ],
        )
    }
    fn get_DFA_visualizer() -> Visualizer {
        Visualizer::from_title_and_fromlist(
            "DFA",
            vec![|ui, reg, _, log| {
                if ui.button("From NFA").clicked() {
                    match &reg.nfa {
                        Some(elem) => {
let elem=DFA::from(elem);
                            let display_graph = DisplayGraph::from(elem.into_graph());
                            reg.dfa = Some(elem);
                            return Some(display_graph);
                        }
                        None => {
                            log.push_back(String::from("Try to visualize a NFA first"));
                        }
                    };
                }
                None
            }],
        )
    }
    fn get_DFA_Minimized_visualizer() -> Visualizer {
        Visualizer::from_title_and_fromlist(
            "Minimized DFA",
            vec![|ui, reg, _, log| {
                if ui.button("From DFA").clicked() {
                    match &reg.dfa {
                        Some(elem) => {
                            let elem=elem.get_minimized_dfa();
                            let display_graph = DisplayGraph::from(elem.into_graph());
                            reg.min_dfa = Some(elem);
                            return Some(display_graph);
                        }
                        None => {
                            log.push_back(String::from("Try to visualize a regular DFA"));
                        }
                    };
                }
                None
            }],
        )
    }

    fn get_visualizers() -> [Visualizer; 4] {
        [
            Self::get_regular_expression_visualizer(),
            Self::get_NFA_visualizer(),
            Self::get_DFA_visualizer(),
            Self::get_DFA_Minimized_visualizer(),
        ]
    }

    pub fn new() -> Self {
        Self {
            object_copy: RegularGrammarObjects::new(),
            error_log: VecDeque::new(),
            visualizers: Self::get_visualizers(),
        }
    }
    pub fn draw_visualizer_windows(&mut self, ctx: &Context) {
        for visualizer in &mut self.visualizers {
            visualizer.display_visualization(ctx);
        }
    }

    pub fn draw_left_panel(&mut self, ui: &mut Ui) {
        for visualizer in &mut self.visualizers {
            visualizer.display_left_panel_graphics(ui, &mut self.object_copy, &mut self.error_log);
        }
        while self.error_log.len() > MAX_LOG_QUEUE {
            self.error_log.pop_front();
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            for error in self.error_log.iter() {
                ui.label(egui::RichText::new(error).color(egui::Color32::RED));
            }
        });
    }
}

//
//
//    pub fn get_converter(index: i32) -> impl Fn(ReOperator) -> Graph {
//        match index {
//            0 => |re: ReOperator| re.into(),
//            1 => |re: ReOperator| NFA::from(&re).into(),
//            2 => |re: ReOperator| DFA::from(&NFA::from(&re)).into(),
//            3 => |re: ReOperator| DFA::from(&NFA::from(&re)).get_minimized_dfa().into(),
//            _ => panic!("Invalid index"),
//        }
//    }

// visualizer.display_left_panel_graphics(ui,);
// if ui
//     .button(format!("Generate {}", visualizer.box_title))
//     .clicked()
// {
//     match ReOperator::from_string(&self.regex_text) {
//         Ok(re) => {
//             visualizer.set_graph(Self::get_converter(index as i32)(re).into());
//             self.error = None;
//         }

//         Err(e) => {
//             self.error = Some(e.to_string());
//         }
//     };
//     ε\":[2,4]},{},{\"a\":[3]},{\"ε\":[1]},{\"b\":[5]},{\"ε\":[1]}],\"used_alphabet\":[\"a\",\"b\"]}"
// }
