use wasm_bindgen::prelude::*;

/// Export functions to WASM
#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
}