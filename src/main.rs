#![allow(non_snake_case)]
#[macro_use]
mod Log;
mod EguiMain;
mod SyntaxTree;
mod RegularExpression;
mod Error;
use EguiMain::EguiApp;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(EguiApp::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(EguiApp::new(cc))),
    )
    .expect("failed to start eframe");
}
