use wasm_bindgen::prelude::*;

const ADD_CONSTANT: i32 = 24;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Exportando una funcion a wasm
#[wasm_bindgen]
pub fn call_me_from_javascript(a: i32, b: i32) -> i32 {
    add_integer_with_constant(a, b)
}

fn add_integer_with_constant(a: i32, b: i32) -> i32 {
    a + b + ADD_CONSTANT
}
