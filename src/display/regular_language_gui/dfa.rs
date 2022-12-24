use crate::{
    automata::DFA,
    display::{DisplayGraph, FromFunction, Visualizer},
    utils::IntoGraph,
};

pub fn get_DFA_visualizer() -> Visualizer {
    Visualizer::from_title_and_fromlist("DFA", vec![visualize_self(), from_nfa()])
}

fn visualize_self() -> FromFunction {
    |ui, grammar_objects, _, _| {
        if let Some(reg) = &grammar_objects.dfa {
            if ui.button("Visuzlize the last DFA Saved").clicked() {
                return Some(DisplayGraph::from(reg.into_graph()));
            }
        }
        None
    }
}

fn from_nfa() -> FromFunction {
    |ui, reg, _, log| {
        if ui.button("From NFA").clicked() {
            match &reg.nfa {
                Some(elem) => {
                    let elem = DFA::from(elem);
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
    }
}
