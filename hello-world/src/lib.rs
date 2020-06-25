// wasm-pack usa wasm-bindgen para construir y generar Javascript binding file
use wasm_bindgen::prelude::*;

const ADD_CONSTANT: i32 = 24;

const WASM_MEMORY_BUFFER_SIZE: usize = 2;

// global y static mut pueden ser unsafe code
// Pero para pasar memoria entre js y wasm podria estar bien
// Se crea un array de 2 posicione sin signo de 8bytes y se lo llena de 0s
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

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

/*
Funcion para guardar el valor pasado en el indice 0 en el buffer
*/
#[wasm_bindgen] 
pub fn store_value_in_wasm_memory_buffer_index_zero(value: u8) {
    unsafe {
        WASM_MEMORY_BUFFER[0] = value;
    }
}

/*
Funcion que retorna el apuntador del buffer en memoria de wasm
*/
#[wasm_bindgen]
pub fn get_wasm_memory_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = WASM_MEMORY_BUFFER.as_ptr();
    }
    pointer
}

/*
Funcion que lee el indice 1 del buffer y retorna el valor del indice
*/
#[wasm_bindgen]
pub fn read_wasm_memory_buffer_and_return_index_one() -> u8 {
    let value: u8;
    unsafe  {
        value = WASM_MEMORY_BUFFER[1];
    }
    value
}

// Importando funciones de javascript
// Definiendo una externa funcion importada desde JS
#[wasm_bindgen]
extern "C" {
    // Usando el js_namespace
    // Se esta bbindiando console.log a log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Exportando la funcion para poderla llamar desde JS
#[wasm_bindgen]
pub fn console_log_from_wasm() {
    log("This console.log from wasm");
}