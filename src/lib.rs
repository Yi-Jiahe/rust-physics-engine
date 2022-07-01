extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern crate nalgebra as na;

pub struct Body2D {
    position: na::Vector2<f32>,
}

impl Body2D {

}

#[wasm_bindgen]
impl Body2D {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}