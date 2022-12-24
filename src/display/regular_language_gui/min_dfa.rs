use crate::{
    display::{DisplayGraph, FromFunction, Visualizer},
    utils::IntoGraph,
};

pub fn get_DFA_Minimized_visualizer() -> Visualizer {
    Visualizer::from_title_and_fromlist("Minimized DFA", vec![visualize_self(), from_dfa()])
}

fn visualize_self() -> FromFunction {
    |ui, grammar_objects, _, _| {
        if let Some(reg) = &grammar_objects.min_dfa {
            if ui.button("Visuzlize the last Min DFA Saved").clicked() {
                return Some(DisplayGraph::from(reg.into_graph()));
            }
        }
        None
    }
}

fn from_dfa() -> FromFunction {
    |ui, reg, _, log| {
        if ui.button("From DFA").clicked() {
            match &reg.dfa {
                Some(elem) => {
                    let elem = elem.get_minimized_dfa();
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
    }
}
