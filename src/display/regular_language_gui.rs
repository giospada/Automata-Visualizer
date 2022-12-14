use crate::automata::{NfaStates, ReOperator, DFA, NFA};
use egui::{Context, Ui};

use std::collections::VecDeque;

use crate::display::Visualizer;

pub type RegularGrammarObjects = (
    Option<ReOperator>,
    Option<NFA>,
    Option<DFA<NfaStates>>,
    Option<DFA<NfaStates>>,
);

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
            vec![|ui, reg, input,log| {
                ui.text_edit_singleline(input);
                if ui.button("Regular Expression").click() {
                    match ReOperator::from_string(input.as_str()) {
                        Ok(elem) => {
                            reg.0 = elem;
                            return elem.into(); 
                        }
                        Err(error) => {
                            log.push_back("Impossibile creare la regular expression");
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
            Visualizer::from_title_and_fromlist("NFA", vec![]),
            Visualizer::from_title_and_fromlist("DFA", vec![]),
            Visualizer::from_title_and_fromlist("DFA Minimize", vec![]),
        ]
    }

    pub fn new() -> Self {
        Self {
            object_copy: (None, None, None, None),
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
// }
