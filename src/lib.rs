use std::collections::HashMap;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

pub mod math;

use crate::math::f32Vector2;

#[wasm_bindgen]
#[derive(Debug)]
pub struct World2D {
    next_id: i32,
    bodies: HashMap<i32, Body2D>,
}

#[wasm_bindgen]
impl World2D {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World2D {
        World2D {
            next_id: 0,
            bodies: HashMap::new(),
        }
    }

    pub fn new_body(&mut self, mass: f32, position: math::f32Vector2) -> Body2D {
        let id = self.next_id;
        self.next_id += 1;
        Body2D::new(id, mass, position)
    }

    pub fn add(&mut self, body: Body2D) {
        self.bodies.insert(body.id, body);
    }

    pub fn update(&mut self, time_delta: f32) {
        for (_id, body) in &mut self.bodies {
            body.update(time_delta);
        }
    }

    pub fn get_body(&self, id: i32) -> Option<&Body2D> {
        self.bodies.get(id)
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Body2D {
    id: i32,
    mass: f32,
    position: f32Vector2,
    velocity: f32Vector2,
    force: f32Vector2,
}

impl Body2D {
    fn new(id: i32, mass: f32, position: math::f32Vector2) -> Body2D {
        Body2D {
            id,
            mass,
            position,
            velocity: math::f32Vector2 { x: 0.0, y: 0.0 },
            force: math::f32Vector2 { x: 0.0, y: 0.0 },
        }
    }

    fn update(&mut self, time_delta: f32) {
        let acceleration = math::f32Vector2 {
            x: self.force.x / self.mass,
            y: self.force.y / self.mass,
        };

        let delta_t_squared = f32::powi(time_delta, 2);
        self.position.x += self.velocity.x * time_delta + 0.5 * acceleration.x * delta_t_squared;
        self.position.y += self.velocity.y * time_delta + 0.5 * acceleration.y * delta_t_squared;

        self.velocity = math::f32Vector2 {
            x: self.velocity.x + acceleration.y * time_delta,
            y: self.velocity.x + acceleration.y * time_delta,
        };
    }
}

#[wasm_bindgen]
impl Body2D {
    pub fn get_position(&self) -> f32Vector2 {
        self.position.clone()
    }

    pub fn apply_force(&mut self, force: f32Vector2) {
        self.force.x += force.x;
        self.force.y += force.y;
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
