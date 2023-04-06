// use std::collections::VecDeque;
//
// use egui::{emath, Frame, Pos2, Rect, Ui, Window};
//
// use crate::display::DisplayGraph;
//
// use super::DisplayGraphParameter;
//
// /// this struct rappresent a visualizer of a graph
// /// it contains the information to show the window and display the graph
//
// /// if Return a String it return an error
// pub type FromFunction = fn(
//     &mut Ui,
//     &mut RegularGrammarObjects,
//     &mut String,
//     &mut VecDeque<String>,
// ) -> Option<DisplayGraph>;
//
// pub struct Visualizer {
//     pub box_title: String,
//     pub graph: Option<DisplayGraph>,
//     pub display_parameters: DisplayGraphParameter,
//     pub is_win_open: bool,
//     pub from: Vec<FromFunction>,
//     pub from_input_string: String,
// }
//
// impl Visualizer {
//     pub fn from_title_and_fromlist(box_title: &str, from_list: Vec<FromFunction>) -> Self {
//         Self {
//             box_title: String::from(box_title),
//             is_win_open: false,
//             display_parameters: DisplayGraphParameter {
//                 padding_x: 40.,
//                 padding_y: 40.,
//                 node_size: 30.,
//             },
//             graph: None,
//             from: from_list,
//             from_input_string: String::new(),
//         }
//     }
//
//     pub fn display_left_panel_graphics(
//         &mut self,
//         ui: &mut Ui,
//         other_obj: &mut RegularGrammarObjects,
//         error_log: &mut VecDeque<String>,
//     ) {
//         ui.heading(self.box_title.clone());
//         for fun in self.from.iter() {
//             if let Some(res) = fun(ui, other_obj, &mut self.from_input_string, error_log) {
//                 self.graph = Some(res);
//                 self.is_win_open = true;
//             }
//         }
//     }
//
//     pub fn check_open(&mut self) {
//         if let None = self.graph {
//             self.is_win_open = false;
//         }
//         if !self.is_win_open {
//             self.graph = None;
//         }
//     }
//
//     fn display_parameters(display_parameters: &mut DisplayGraphParameter, ui: &mut Ui) {
//         let par = display_parameters;
//         ui.collapsing(format!("visualizer option",), |ui| {
//             ui.add(egui::Slider::new(&mut par.padding_x, 10.0..=100.0).text("padding x"));
//             ui.add(egui::Slider::new(&mut par.padding_y, 10.0..=100.0).text("padding y"));
//             ui.add(egui::Slider::new(&mut par.node_size, 10.0..=100.0).text("node size"));
//         });
//     }
//
//     fn display_graph(
//         graph: &mut Option<DisplayGraph>,
//         display_parameters: DisplayGraphParameter,
//         ui: &mut Ui,
//     ) {
//         Frame::canvas(ui.style()).show(ui, |canvas_ui| {
//             if let Some(tree) = graph {
//                 let scren_size = tree.position(display_parameters);
//                 let (mut response, painter) =
//                     canvas_ui.allocate_painter(scren_size, egui::Sense::hover());
//
//                 let to_screen = emath::RectTransform::from_to(
//                     Rect::from_min_size(Pos2::ZERO, response.rect.size()),
//                     response.rect,
//                 );
//                 tree.drag_nodes(to_screen, canvas_ui, &mut response);
//                 tree.draw(&painter, to_screen, canvas_ui);
//             }
//         });
//     }
//
//     pub fn display_visualization(&mut self, ctx: &egui::Context) {
//         self.check_open();
//         let syntaxTree = Window::new(format!("{}", self.box_title));
//         let syntaxTree = syntaxTree.open(&mut self.is_win_open);
//         let syntaxTree = syntaxTree.scroll2([true, true]);
//         syntaxTree.show(ctx, |ui| {
//             Self::display_parameters(&mut self.display_parameters, ui);
//             Self::display_graph(&mut self.graph, self.display_parameters, ui);
//         });
//     }
// }
