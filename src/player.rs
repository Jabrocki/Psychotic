use nalgebra::{Rotation2, Vector2};

use crate::{entities::Entity, entity::unit::UnitEntity, world::World};

pub struct Player {
    position: Vector2<f32>,
    direction: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    hit_box: Vector2<f32>,

    hp: i32,
    max_hp: i32,
    atk: i32,
    df: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(1.5, 1.5),
            direction: Vector2::new(-1.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            hit_box: Vector2::new(0.1, 0.1),

            hp: 30,
            max_hp: 30,
            atk: 2,
            df: 0,
        }
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn set_position(&mut self, pos: Vector2<f32>) {
        self.position = pos;
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

    fn update(&mut self, world: &mut World, delta_time: f32) {
        self.velocity += self.acceleration * delta_time;
        let new_position_x = self.position.x + self.velocity.x * delta_time;
        if !world.wall_collsion(Vector2::new(new_position_x, self.position.y), self.hit_box) {
            self.position.x = new_position_x;
            self.velocity.x *= 0.8;
        } else {
            self.velocity.x = 0.0;
        }

        let new_position_y = self.position.y + self.velocity.y * delta_time;
        if !world.wall_collsion(Vector2::new(self.position.x, new_position_y), self.hit_box) {
            self.position.y = new_position_y;
            self.velocity.y *= 0.8;
        } else {
            self.velocity.y = 0.0;
        }
        self.acceleration = Vector2::new(0.0, 0.0);
    }

    fn sprite_id(&self) -> Option<u8> {
        None
    }
}

impl UnitEntity for Player{
    fn set_hp(&mut self, hp: i32) {
    	self.hp = hp;
    }

    fn hp(&self) -> i32 {
    	self.hp
    }

    fn set_max_hp(&mut self, max_hp: i32) {
    	self.max_hp = max_hp;
    }

    fn max_hp(&self) -> i32 {
    	self.max_hp
    }

    fn atk(&self) -> i32 {
    	self.atk
    }

    fn df(&self) -> i32 {
    	self.df
    }
}
