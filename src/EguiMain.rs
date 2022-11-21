use eframe::egui;
use egui::{emath, Frame, Pos2, Rect, Sense, Window,Color32,RichText};
use crate::SyntaxTree::*;
use crate::RegularExpression::*;
use crate::RegexVisualizer::*;

use crate::Log::*;

pub struct EguiApp {
    re:RegexVisualizer,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            re:RegexVisualizer::new(),
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
        egui::SidePanel::left("Main").show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y=6.; 
            ui.heading("Regular Expression");
            ui.horizontal(
                |ui | {
                    ui.label("inserisci la regex");
                    ui.text_edit_singleline(&mut self.re.regex_text).on_hover_text("Enter a regular expression");
                }
            );

            if ui.button("Generate SyntaxTree").clicked() { 
                self.re.generate_tree();                
            }

            if let Some(err)= &self.re.regex_error {
                ui.label(RichText::new(err).color(Color32::RED));
            }

            ui.collapsing("syntax tree visualization option",|ui|{
                ui.add(egui::Slider::new(&mut self.re.padding_x,10.0..=100.0).text("padding x:"));
                ui.add(egui::Slider::new(&mut self.re.padding_y,10.0..=100.0).text("padding y:"));
                ui.add(egui::Slider::new(&mut self.re.size_node,10.0..=100.0).text("node size:"));
            });
        });

        self.re.check_open();
        let syntaxTree= Window::new("SyntaxTree");
        let syntaxTree = syntaxTree.open(&mut self.re.open);
        let syntaxTree = syntaxTree.scroll2([true,true]);
        syntaxTree.show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                if let Some(tree) = &mut self.re.tree { 
                    // va ricalcolato solo se abbiamo cambiato i parametri quindi possiamo cachare
                    // todo!(" ");
                    let scren_size=
                        tree.position_tree(Pos2{x:self.re.padding_x,y:self.re.padding_y},self.re.size_node);
                    let (mut response, painter) =
                        ui.allocate_painter(scren_size, egui::Sense::drag());

                    let to_screen = emath::RectTransform::from_to(
                        Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                        response.rect,
                    ); 

                    tree.draw_tree(&painter, to_screen, &ui, &mut response);
                }
            })
        });
    }
}
