extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct f32Vector2(pub f32, pub f32);
