#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
#[cfg(not(target_arch = "wasm32"))]
pub fn log(s: &str) {
    println!("{}", s);
}

#[macro_export]
macro_rules!  log{
    ($($t:tt)*) => (log(&format!($($t)*)))
}
