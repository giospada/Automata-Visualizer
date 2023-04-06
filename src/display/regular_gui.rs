use egui::{emath, Frame, Pos2, Rect, Ui};

use crate::automata::{NfaStates, ReOperator, DFA, NFA};

use crate::utils::{Graph, IntoGraph};

use super::{DisplayGraph, DisplayGraphParameter};

pub struct RegularGui {
    dfa: Vec<(String, DFA<NfaStates>)>,
    nfa: Vec<(String, NFA)>,
    regex: Vec<(String, ReOperator)>,
    current_workspace: CurrentWorkspace,
    object_created: i32,
}

enum CurrentWorkspace {
    NFA(NFA),
    DFA(DFA<NfaStates>),
    REGEX(ReOperator),
}

impl RegularGui {
    pub fn new() -> Self {
        return RegularGui {
            dfa: Vec::new(),
            nfa: Vec::new(),
            regex: Vec::new(),
            current_workspace: CurrentWorkspace::REGEX(ReOperator::Char('a')),
            object_created: 0,
        };
    }

    pub fn draw_left_panel(&mut self, ui: &mut Ui) {}

    pub fn center_panel(&mut self, ui: &mut Ui) {
        let tree: Graph = match &self.current_workspace {
            CurrentWorkspace::NFA(nfa) => nfa.into_graph(),
            CurrentWorkspace::DFA(dfa) => dfa.into_graph(),
            CurrentWorkspace::REGEX(re) => re.into_graph(),
        };
        let mut tree: DisplayGraph = tree.into();
        Frame::canvas(ui.style()).show(ui, |canvas_ui| {
            let scren_size = tree.position(DisplayGraphParameter {
                padding_x: 40.,
                padding_y: 40.,
                node_size: 30.,
            });
            let (mut response, painter) =
                canvas_ui.allocate_painter(scren_size, egui::Sense::hover());

            let to_screen = emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );
            tree.drag_nodes(to_screen, canvas_ui, &mut response);
            tree.draw(&painter, to_screen, canvas_ui);
        });
    }

    pub fn draw_right_panel(&mut self, ui: &mut Ui) {
        self.draw_re_operator_menu(ui);
        self.draw_nfa_menu(ui);
        self.draw_dfa_menu(ui);
    }

    fn gen_snapshot_row<F1, F2>(ui: &mut Ui, title: &mut String, load: F1, menu_fun: F2)
    where
        F1: Fn(),
        F2: FnOnce(&mut Ui),
    {
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(title).desired_width(120.0));
            ui.menu_button("Option", menu_fun);
            if ui.button("load").clicked() {
                load();
            }
        });
    }

    fn draw_nfa_menu(&mut self, ui: &mut Ui) {
        ui.heading("NFA");
        if ui.button("New").clicked() {
            self.nfa.push((
                String::from("NFA ") + self.object_created.to_string().as_str(),
                NFA::new(),
            ));
            self.object_created += 1;
        }
        self.nfa.iter_mut().for_each(|(s, _obj)| {
            Self::gen_snapshot_row(
                ui,
                s,
                || {},
                |ui| {
                    if ui.button("Open...").clicked() {
                        ui.close_menu();
                    }
                },
            )
        });
    }
    fn draw_dfa_menu(&mut self, ui: &mut Ui) {
        ui.heading("DFA");
        if ui.button("New").clicked() {
            self.object_created += 1;
        }
        self.dfa.iter_mut().for_each(|(s, _obj)| {
            Self::gen_snapshot_row(
                ui,
                s,
                || {},
                |ui| {
                    if ui.button("Open...").clicked() {
                        ui.close_menu();
                    }
                },
            )
        });
    }
    fn draw_re_operator_menu(&mut self, ui: &mut Ui) {
        ui.heading("ReOperator");
        if ui.button("New").clicked() {
            self.regex.push((
                String::from("Regex ") + self.object_created.to_string().as_str(),
                ReOperator::Char('a'),
            ));
            self.object_created += 1;
        }
        self.regex.iter_mut().for_each(|(s, _obj)| {
            Self::gen_snapshot_row(
                ui,
                s,
                || {},
                |ui| {
                    if ui.button("Open...").clicked() {
                        ui.close_menu();
                    }
                },
            )
        });
    }
}
