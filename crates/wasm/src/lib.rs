use {{project-name | snake_case}}_core::add;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_wasm(a: usize, b: usize) -> usize {
    add(a, b)
}
