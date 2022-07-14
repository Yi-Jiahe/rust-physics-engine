extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct f32Vector2(pub f32, pub f32);
