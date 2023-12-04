use wasm_bindgen::prelude::*;
// use serde_json::*;

use std::ffi::CString;
use std::os::raw::c_char;



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

// #[wasm_bindgen]
// pub fn hello(name: &str) -> JsValue {
//     JsValue::from_str(&format!("Hello from rust, {}!", name))
// }



// #[wasm_bindgen]
// pub fn hello2(name: &str) -> String {
//     format!("Hello from rust, {}!", name)
// }

// #[wasm_bindgen]
// pub fn hello3(name: &str) -> i64 {
//     let s = format!("Hello from rust, {}!", name);

//     let len = s.len() as i64;
//     let _s = CString::new(s).unwrap();
//     let a = _s.into_raw() as i64;

//     // len * (2^32) + a
//     a * 100000 + len
// }



// use wasm_bindgen::prelude::*;

// Our function to concatenate the string "Wasm by Example"
// to the input string. We are using .into(), to convert
// the rust types of str to a String.
#[wasm_bindgen]
pub fn add_wasm_by_example_to_string(input_string: String) -> String {
  let result = format!("{} {}", input_string, "Wasm by Example");
  return result.into();
}


// #[wasm_bindgen]
// pub fn get_position_str() -> js_sys::Uint32Array {
//     // unsafe { serde_json::to_string(&POSITION).unwrap() }
//     // JsValue::from_str("asdfasdf")
//     // "23423432412".to_owned()
//     let v: Vec<u32> = vec![1, 2, 3];

//     js_sys::Uint32Array::from(&v[..])

// }


// #[wasm_bindgen]
// pub fn get_position_int() -> i32 {
//     // unsafe { serde_json::to_string(&POSITION).unwrap() }
//     // JsValue::from_str("asdfasdf")
//     // "23423432412".to_owned()
//     12

// }



// use std::ffi::CString;
// use std::os::raw::c_char;

// static HELLO: &'static str = "hello from rust";

// #[wasm_bindgen]
// #[no_mangle]
// pub fn get_hello() -> *mut c_char {
//     let s = CString::new(HELLO).unwrap();
//     s.into_raw()
// }

// #[wasm_bindgen]
// #[no_mangle]
// pub fn get_hello_len() -> usize {
//     HELLO.len()
// }


