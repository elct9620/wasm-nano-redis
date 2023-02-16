#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_nano_redis::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn execute_command_success() {
    let srv = server::Server::new();
    assert_eq!(srv.execute("GET name".into()), true);
}
