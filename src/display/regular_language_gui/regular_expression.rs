use crate::{
    automata::ReOperator,
    display::{DisplayGraph, FromFunction, Visualizer},
    utils::IntoGraph,
};

pub fn get_regular_expression_visualizer() -> Visualizer {
    Visualizer::from_title_and_fromlist("Regular Expression", vec![visualize_self(), from_input()])
}

fn visualize_self() -> FromFunction {
    |ui, grammar_objects, _, _| {
        if let Some(reg) = &grammar_objects.regular_expression {
            if ui
                .button("Visuzlize the last Regular Expression Saved")
                .clicked()
            {
                return Some(DisplayGraph::from(reg.into_graph()));
            }
        }
        None
    }
}

fn from_input() -> FromFunction {
    |ui, reg, input, log| {
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
    }
}
