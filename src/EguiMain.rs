use eframe::egui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[cfg(not(target_arch = "wasm32"))]
fn log(s: &str){
    println!("{}",s);
}

use super::CoordUtil::CoordinateSystem;

macro_rules! log{
    ($($t:tt)*) => (log(&format!($($t)*)))
}



pub struct EguiApp {
}

impl Default for EguiApp {
    fn default() -> Self{
        Self{
        }
    }
}


impl EguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}



impl eframe::App for EguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let pointer_pos={ctx.input().pointer.hover_pos()};
        egui::Window::new("my_left_subpanel").show(ctx, |ui| {
            ui.label("sub_panel");
            if {ctx.wants_pointer_input()} {
                ui.label(format!("{:?}",pointer_pos));
            }
        });
        
        egui::Window::new("area").resizable(false).show(ctx, |ui| {
            let painter=ui.painter();
            use egui::Color32;
            let coord=CoordinateSystem::from_clip_rect(painter.clip_rect());

            
            if {ctx.wants_pointer_input()} && coord.in_area_option(pointer_pos) {
                log!("{:?}",pointer_pos);
            }
            
            painter.circle_filled(
                coord.to_cord(coord.max_x()/2.0,coord.max_y()/2.0), 
                coord.max_y()/2.0, 
                Color32::from_rgb(1,2,3)
            );
            ui.allocate_space(ui.available_size()); // put this LAST in your panel/window code
        });
    }
}
