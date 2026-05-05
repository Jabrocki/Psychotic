use nalgebra::{Rotation2, Vector2};

use crate::{entity_manager::Entity, world::World};

pub struct Player {
    position: Vector2<f32>,
    direction: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(1.0, 1.0),
            direction: Vector2::new(-1.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
        }
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn direction(&self) -> Vector2<f32> {
        self.direction
    }

    pub fn up(&mut self) {
        self.acceleration += self.direction * 50.0;
    }

    pub fn down(&mut self) {
        self.acceleration -= self.direction * 50.0;
    }

    pub fn left(&mut self) {
        let left_direction = Vector2::new(self.direction.y, -self.direction.x);
        self.acceleration += left_direction * 50.0;
    }

    pub fn right(&mut self) {
        let right_direction = Vector2::new(-self.direction.y, self.direction.x);
        self.acceleration += right_direction * 50.0;
    }

    pub fn rotate(&mut self, angle: f32) {
        let rotation_matrix: Rotation2<f32> = Rotation2::new(angle);
        self.direction = rotation_matrix * self.direction;
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity for Player {
    fn position(&self) -> Vector2<f32> {
        self.position
    }

    fn set_position(&mut self, pos: Vector2<f32>) {
        self.position = pos;
    }

    fn update(&mut self, entities: &mut World, delta_time: f32) {
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
        self.velocity *= 0.8; // Friction
        self.acceleration = Vector2::new(0.0, 0.0);
    }

    fn sprite_id(&self) -> Option<u8> {
        None
    }
}
