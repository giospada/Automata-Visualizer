use eframe::egui;
use egui::SidePanel;

use crate::display::RegularLanguageGui;



pub struct EguiApp {
    leftpanel:RegularLanguageGui,
}

impl Default for EguiApp {
    fn default() -> Self {
        Self {
            leftpanel:RegularLanguageGui::new()

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
        SidePanel::left("Main").show(ctx, |ui| {
            self.leftpanel.draw_left_panel(ui);
        });
       self.leftpanel.draw_visualizer_windows(ctx); 
    }
}
