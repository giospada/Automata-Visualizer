#![allow(non_snake_case)]

mod app;
mod utils;
mod display;
mod automata;
mod error;

use app::EguiApp;

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
    console_log::init_with_level(Level::Debug);

    let mut web_options = eframe::WebOptions::default();
    web_options.default_theme = eframe::Theme::Dark;
    web_options.follow_system_theme = false;
    
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(EguiApp::new(cc))),
    )
    .expect("failed to start eframe");
}
