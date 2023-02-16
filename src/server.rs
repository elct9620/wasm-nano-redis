use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Server {}

#[wasm_bindgen]
impl Server {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Server {
        Server {}
    }

    pub fn execute(&self, _command: String) -> bool {
        true
    }
}
