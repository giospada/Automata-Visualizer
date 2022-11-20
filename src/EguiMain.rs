use eframe::egui;
use egui::{emath, Frame, Pos2, Rect, Sense, Window,Color32,RichText};
use crate::SyntaxTree::*;
use crate::RegularExpression::*;

use crate::Log::*;

pub struct EguiApp {
    tree: SyntaxTree,
    regex_text: String,
    parser_error:Option<String>
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            tree: ReOperator::Char('a').to_syntax_tree(),
            regex_text: String::new(),
            parser_error:None
        }
    }
}

impl EguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let _pointer_pos = { ctx.input().pointer.hover_pos() };
        Window::new("settings panel").show(ctx, |ui| {
            ui.horizontal(
                |ui | {
                    ui.label("inserisci la regex");
                    // let response = ui.add(egui::TextEdit::singleline(&mut regex_text));
                    let response = ui.text_edit_singleline(&mut self.regex_text).on_hover_text("Enter a regular expression");

                    if response.lost_focus() {
                        let tree = ReOperator::from_string(self.regex_text.clone());
                        match tree {
                            Ok(tree) => {
                                self.tree = tree.to_syntax_tree();
                                self.parser_error=None;
                            },
                            Err(err) =>{
                                self.parser_error=Some(err.to_string());
                            }
                        };
                        //if let Ok(tree) = tree {
                            //self.tree = tree.to_syntax_tree();
                        //}
                        // TODO: display error message if there is in window
                    }
                }
            );
            if let Some(err)= &self.parser_error {
                ui.label(RichText::new(err).color(Color32::RED));
            }
            if ui.button("test log").clicked() {
                log!("test log");
            }
        });

        Window::new("Canvaxas").resizable(false).show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                let (mut response, painter) =
                    ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

                let to_screen = emath::RectTransform::from_to(
                    Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
                    response.rect,
                );
                let _from_screen = to_screen.inverse();
                let mh = painter.clip_rect().height();
                let mw = painter.clip_rect().width();
                let top_padding = painter.clip_rect().min;
                log!("mh: {}, mw: {}", mh, mw);
                self.tree.position_tree(Pos2::new(mw, mh));
                self.tree
                    .draw_tree(&painter, top_padding.to_vec2(), &ui, &mut response);
            });
        });
    }
}
