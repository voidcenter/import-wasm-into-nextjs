use wasm_bindgen::prelude::*;

static mut POSITION: i32 = 100;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn get_position() -> i32 {
    unsafe { POSITION }
}

#[wasm_bindgen]
pub fn perform_command(command: i32) {
    if command == 0 {
        unsafe { POSITION -= 1 }
    } else {
        unsafe { POSITION += 1 }
    }
}

