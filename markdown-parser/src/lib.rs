use wasm_bindgen::prelude::*;

mod parser;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let result = parser::parse(input.to_string());
    result
}