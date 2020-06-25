use wasm_bindgen::prelude::*;

// Definiendo el tamaño del tablero
const CHECKBOARD_SIZE: usize = 20;

// 20x20 pixeles y 4 colores por pixel (r,g,b,a) lo que soporta canvas
const OUTPUT_BUFFER_SIZE: usize = CHECKBOARD_SIZE * CHECKBOARD_SIZE * 4;

// Esta sera utilizada para poner la salida de nuestro grafico y pasarla a la salida de JS
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

/*
Funcion que retorna un puntero del buffer en wasm memory
*/
#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = OUTPUT_BUFFER.as_ptr();
    }
    pointer
}

// Funcion que genera un tablero, pixel a pixel
#[wasm_bindgen]
pub fn generate_checker_board(
    dark_value_red: u8,
    dark_value_green: u8,
    dark_value_blue: u8,
    light_value_red: u8,
    light_value_green: u8,
    light_value_blue: u8
) {
    for y in 0..CHECKBOARD_SIZE {
        for x in 0..CHECKBOARD_SIZE {
            // Por defecto el cuadro negro
            let mut is_dark_square: bool = true;

            // Cambiando el el valor por defecto en el caso que sea par
            if y % 2 == 0 {
                is_dark_square = false;
            }

            // Alternando el valor si x es par
            if x % 2 == 0 {
                is_dark_square = !is_dark_square;
            }

            // Determinando si el cuadro es oscuro o claro
            let mut square_value_red: u8 = dark_value_red;
            let mut square_value_green: u8 = dark_value_green;
            let mut square_value_blue: u8 = dark_value_blue;

            if !is_dark_square {
                square_value_red = light_value_red;
                square_value_green = light_value_green;
                square_value_blue = light_value_blue
            }

            // Calculando el indice de nuestro 2D en 1D mapping
            let square_numer: usize = y * CHECKBOARD_SIZE + x;
            let square_rgba_index: usize = square_numer * 4;

            // Guardamos los valores
            unsafe {
                OUTPUT_BUFFER[square_rgba_index + 0] = square_value_red;
                OUTPUT_BUFFER[square_rgba_index + 1] = square_value_green;
                OUTPUT_BUFFER[square_rgba_index + 2] = square_value_blue;
                OUTPUT_BUFFER[square_rgba_index + 3] = 255; // Alpha
            }
        }
    }
}