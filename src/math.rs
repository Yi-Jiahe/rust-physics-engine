extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct f32Vector2 {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen]
impl f32Vector2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> f32Vector2 {
        f32Vector2 {
            x,
            y,
        }
    }
}
