use crate::{
    automata::NFA,
    display::{DisplayGraph, FromFunction, Visualizer},
    utils::IntoGraph,
};

pub fn get_NFA_visualizer() -> Visualizer {
    Visualizer::from_title_and_fromlist(
        "NFA",
        vec![
            visualize_self(),
            from_input_text(),
            from_regular_expression(),
        ],
    )
}

fn visualize_self() -> FromFunction {
    |ui, grammar_objects, _, _| {
        if let Some(reg) = &grammar_objects.nfa {
            if ui.button("Visuzlize the last NFA Saved").clicked() {
                return Some(DisplayGraph::from(reg.into_graph()));
            }
        }
        None
    }
}
fn from_regular_expression() -> FromFunction {
    |ui, reg, _, log| {
        if ui.button("From Regular Expression").clicked() {
            match &reg.regular_expression {
                Some(elem) => {
                    let elem = NFA::from(elem);
                    let display_graph = DisplayGraph::from(elem.into_graph());
                    reg.nfa = Some(elem);
                    return Some(display_graph);
                }
                None => {
                    log.push_back(String::from("Try to visualize a regular Expression First"));
                }
            };
        }
        None
    }
}
fn from_input_text() -> FromFunction {
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
    }
}
