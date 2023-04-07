use eframe::egui;
use egui::{panel::TopBottomSide, CentralPanel, SidePanel, TopBottomPanel};

use crate::display::RegularGui;

pub struct EguiApp {
    regular_gui: RegularGui,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            regular_gui: RegularGui::new(),
        }
    }
}

impl EguiApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("Left").show(ctx, |ui| {
            self.regular_gui.draw_left_panel(ui);
        });
        SidePanel::right("Right").show(ctx, |ui| {
            self.regular_gui.draw_right_panel(ui);
        });
        TopBottomPanel::bottom("Visualizer").show(ctx, |ui| {
            self.regular_gui.draw_bottom_panel(ui);
        });
        CentralPanel::default().show(ctx, |ui| {
            self.regular_gui.center_panel(ui);
        });
    }
}
