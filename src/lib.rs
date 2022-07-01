extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern crate nalgebra as na;

pub struct Body2D {
    mass: f32,
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
    acceleration: na::Vector2<f32>,
}

impl Body2D {
    fn update(&mut self, time_delta: f32) {
        self.velocity *= na::Vector1::new(f32::powi(time_delta, 2));
        self.position += self.velocity * na::Vector1::new(time_delta);
    }
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