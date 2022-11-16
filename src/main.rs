use eframe::egui;
use eframe::egui::Pos2;


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .expect("failed to start eframe");
}


#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

struct CoordinateSystem{
    top:Pos2,
    bottom:Pos2
}

impl CoordinateSystem{
    fn from_clip_rect(clip_rect:egui::Rect) -> Self{
        Self { top: (clip_rect.min), bottom: (clip_rect.max) } 
    }
    fn max_x(&self) -> f32{
        self.bottom.x-self.top.x
    }
    fn max_y(&self) -> f32{
        self.bottom.y-self.top.y
    }
    fn to_acctual_coord(&self,coord:&Pos2)->Pos2{
        (self.top+coord.to_vec2())
            .clone()
    }
    fn to_cord(&self,x:f32,y:f32)->Pos2{
        Pos2{
            x:self.top.x+x,
            y:self.top.y+y
        }
    }
}


impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::Window::new("my_left_subpanel").show(ctx, |ui| {
            ui.label("sub_panel");
        });
        egui::Area::new("area").show(ctx, |ui| {
            let painter=ui.painter();
            use egui::Color32;
            let coord=CoordinateSystem::from_clip_rect(painter.clip_rect());
            painter.circle_filled(
                coord.to_cord(coord.max_x()/2.0,coord.max_y()/2.0), 
                coord.max_y()/2.0, 
                Color32::from_rgb(1,2,3)
            );
        });
    }
}
