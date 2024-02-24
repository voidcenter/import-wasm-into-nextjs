
use serde::*;
// use wasm_bindgen::prelude::*;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[wasm_bindgen]
pub struct  Vector2 {
    pub x: i32,
    pub y: i32,
}


// #[wasm_bindgen]
impl Vector2 {
    pub fn new(x: i32, y: i32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn add(&mut self, src: &Vector2) {
        self.x += src.x;
        self.y += src.y;
    }

    pub fn mul(&mut self, fac: i32) {
        self.x *= fac;
        self.y *= fac;
    }

    // #[wasm_bindgen(getter)]
    pub fn zero() -> Vector2 {
        Vector2::ZERO
    }

    // #[wasm_bindgen(getter)]
    pub fn right() -> Vector2 {
        Vector2::RIGHT
    }

    // #[wasm_bindgen(getter)]
    pub fn left() -> Vector2 {
        Vector2::LEFT
    }

    // #[wasm_bindgen(getter)]
    pub fn up() -> Vector2 {
        Vector2::UP
    }

    // #[wasm_bindgen(getter)]
    pub fn down() -> Vector2 {
        Vector2::DOWN
    }

    // #[wasm_bindgen(getter)]
    pub fn one() -> Vector2 {
        Vector2::ONE
    }
}


impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0, y: 0 };
    const RIGHT: Vector2 = Vector2 { x: 1, y: 0 };
    const LEFT: Vector2 = Vector2 { x: -1, y: 0 };
    const UP: Vector2 = Vector2 { x: 0, y: -1 };
    const DOWN: Vector2 = Vector2 { x: 0, y: 1 };
    const ONE: Vector2 = Vector2 { x: 1, y: 1 };
}



fn main() {
}
