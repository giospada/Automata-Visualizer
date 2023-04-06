use egui::{Response, Ui};

use crate::automata::{NfaStates, ReOperator, DFA, NFA};

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

    pub fn center_panel(&mut self, ui: &mut Ui) {}

    pub fn draw_right_panel(&mut self, ui: &mut Ui) {
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

    fn gen_snapshot_row<F1, F2>(ui: &mut Ui, title: &mut String, load: F1, menu_fun: F2)
    where
        F1: Fn(),
        F2: FnOnce(&mut Ui),
    {
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(title).desired_width(120.0));
            if ui.button("load").clicked() {
                load();
            }
            ui.button("⋮").menu_button(menu_fun);
        });
    }
}
