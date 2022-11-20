use eframe::egui;
use egui::{emath, Frame, Pos2, Rect, Sense, Window};
use crate::SyntaxTree::*;
use crate::RegularExpression::*;
#[macro_use]
use crate::Log::*;

pub struct EguiApp {
    tree: SyntaxTree,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            tree: ReOperator::Or(
                Box::new(ReOperator::Concat(
                    Box::new(ReOperator::Concat(
                        Box::new(ReOperator::Char('a')),
                        Box::new(ReOperator::Char('c')),
                    )),
                    Box::new(ReOperator::Concat(
                        Box::new(ReOperator::Char('f')),
                        Box::new(ReOperator::Char('g')),
                    )),
                )),
                Box::new(ReOperator::Char('b')),
            )
            .to_syntax_tree(),
        }
    }
}

impl EguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let pointer_pos = { ctx.input().pointer.hover_pos() };
        Window::new("settings panel").show(ctx, |ui| {
            ui.horizontal(
                |ui | {
                    ui.label("inserisci la regex");
                    let mut text="b|acfg";
                    ui.text_edit_singleline(&mut text).on_hover_text("Enter a regular expression");
                }
            );
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
                let from_screen = to_screen.inverse();
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
