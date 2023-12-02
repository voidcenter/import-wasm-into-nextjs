use wasm_bindgen::prelude::*;
// use serde_json::*;


// static mut POSITION: i32 = 100;

// #[wasm_bindgen]
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[wasm_bindgen]
// pub fn get_position() -> i32 {
//     unsafe { POSITION }
// }

// #[wasm_bindgen]
// pub fn perform_command(command: i32) {
//     if command == 0 {
//         unsafe { POSITION -= 1 }
//     } else {
//         unsafe { POSITION += 1 }
//     }
// }



// #[wasm_bindgen]
// pub fn get_position_str() -> js_sys::Uint32Array {
//     // unsafe { serde_json::to_string(&POSITION).unwrap() }
//     // JsValue::from_str("asdfasdf")
//     // "23423432412".to_owned()
//     let v: Vec<u32> = vec![1, 2, 3];

//     js_sys::Uint32Array::from(&v[..])

// }


#[wasm_bindgen]
pub fn get_position_int() -> i32 {
    // unsafe { serde_json::to_string(&POSITION).unwrap() }
    // JsValue::from_str("asdfasdf")
    // "23423432412".to_owned()
    12

}



use std::ffi::CString;
use std::os::raw::c_char;

static HELLO: &'static str = "hello from rust";

#[wasm_bindgen]
#[no_mangle]
pub fn get_hello() -> *mut c_char {
    let s = CString::new(HELLO).unwrap();
    s.into_raw()
}

#[wasm_bindgen]
#[no_mangle]
pub fn get_hello_len() -> usize {
    HELLO.len()
}


