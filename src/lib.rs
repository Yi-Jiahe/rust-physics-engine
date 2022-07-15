extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

pub mod math;

use crate::math::f32Vector2;

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
    position: f32Vector2,
    velocity: f32Vector2,
    force: f32Vector2,
}

impl Body2D {
    fn update(&mut self, time_delta: f32) {
        let acceleration = math::f32Vector2 {
            x: self.force.x / self.mass,
            y: self.force.y / self.mass,
        };

        let delta_t_squared = f32::powi(time_delta, 2);
        self.position = math::f32Vector2 {
            x: self.velocity.x * time_delta + 0.5 * acceleration.x * delta_t_squared,
            y: self.velocity.y * time_delta + 0.5 * acceleration.y * delta_t_squared,
        };
        self.velocity = math::f32Vector2 {
            x: self.velocity.x + acceleration.y * time_delta,
            y: self.velocity.x + acceleration.y * time_delta,
        };
    }
}

#[wasm_bindgen]
impl Body2D {
    #[wasm_bindgen(constructor)]
    pub fn new(mass: f32, position: math::f32Vector2) -> Body2D {
        Body2D {
            mass,
            position,
            velocity: math::f32Vector2 { x: 0.0, y: 0.0 },
            force: math::f32Vector2 { x: 0.0, y: 0.0 },
        }
    }

    pub fn get_position(&self) -> f32Vector2 {
        self.position.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_position_from_body() {
        let body = Body2D::new(1.0, math::f32Vector2 { x: 1.2, y: 3.4 });
        let position = body.get_position();
        assert_eq!(position.x, 1.2);
        assert_eq!(position.y, 3.4);
    }

    #[test]
    fn it_works() {
        let mut world = World2D::new();
        world.add(Body2D::new(1.0, math::f32Vector2 { x: 0.0, y: 0.0 }));

        world.update(0.1);

        dbg!(world);
    }
}
