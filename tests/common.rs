#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_nano_redis::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn version_is_configured() {
    assert_eq!(version(), "0.1.0");
}
