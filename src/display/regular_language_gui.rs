use crate::automata::{NfaStates, ReOperator, DFA, NFA};
use egui::{Context, Ui};

use std::collections::VecDeque;

use crate::display::Visualizer;

mod dfa;
mod min_dfa;
mod nfa;
mod regular_expression;

use dfa::get_DFA_visualizer;
use min_dfa::get_DFA_Minimized_visualizer;
use nfa::get_NFA_visualizer;
use regular_expression::get_regular_expression_visualizer;

/// This struct contains re,nfa,dfa,min dfa (maybe in a funture a regular grammar),
/// so it store all the object diplayed so we can transform object to each other
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

/// It's the menu where you can modify the regular object conteinded in `RegularGrammarObjects`
/// and transoform each object to others
pub struct RegularLanguageGui {
    object_copy: RegularGrammarObjects,
    error_log: VecDeque<String>,
    visualizers: [Visualizer; 4],
}

const MAX_LOG_QUEUE: usize = 5;

impl RegularLanguageGui {
    pub fn new() -> Self {
        Self {
            object_copy: RegularGrammarObjects::new(),
            error_log: VecDeque::new(),
            visualizers: get_visualizers(),
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

        ui.label(format!("currently you have saved in memory: "));
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

fn get_visualizers() -> [Visualizer; 4] {
    [
        get_regular_expression_visualizer(),
        get_NFA_visualizer(),
        get_DFA_visualizer(),
        get_DFA_Minimized_visualizer(),
    ]
}
