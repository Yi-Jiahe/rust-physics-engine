extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern crate nalgebra as na;

#[wasm_bindgen]
#[derive(Debug)]
pub struct World2D {
    bodies: Vec<Body2D>,
}

#[wasm_bindgen]
impl World2D {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World2D {
        World2D {
            bodies: Vec::<Body2D>::new(),
        }
    }

    pub fn add(&mut self, body: Body2D) {
        self.bodies.push(body);
    }

    pub fn update(&mut self, time_delta: f32) {
        for body in &mut self.bodies {
            body.update(time_delta);
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
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
    #[wasm_bindgen(constructor)]
    pub fn new(mass: f32, position: Vec<f32>) -> Body2D {
        Body2D {
            mass,
            position: na::point![position[0], position[1]],
            velocity: na::vector![0.0, 0.0],
            acceleration: na::vector![0.0, 0.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut world = World2D::new();
        world.add(Body2D::new(1.0, vec![0.0, 0.0]));

        world.update(0.1);

        dbg!(world);
    }
}