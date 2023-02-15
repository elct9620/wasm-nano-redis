mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn version() -> String {
    "0.1.0".into()
}
