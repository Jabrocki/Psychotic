use nalgebra::Vector2;

use crate::{entities::Entity, entity::unit::UnitEntity, movement::Moveable};

pub struct Enemy{
	position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    hit_box: Vector2<f32>,

    hp: i32,
    max_hp: i32,
    atk: i32,
    df: i32,
}

impl Entity for Enemy {
    fn position(&self) -> Vector2<f32> {
        self.position
    }

    fn set_position(&mut self, pos: nalgebra::Vector2<f32>) {
    	self.position = pos;
    }

    fn update(&mut self, world: &mut crate::world::World, delta_time: f32) {
    	self.simulate_movement(world, delta_time);
    }

    fn sprite_id(&self) -> Option<u8> {
    	Some(11)
    }
}

impl UnitEntity for Enemy {
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

impl Moveable for Enemy{
    fn set_acceleration(&mut self, acceleration: Vector2<f32>) {
    	self.acceleration = acceleration;
    }

    fn acceleration(&self) -> Vector2<f32> {
    	self.acceleration
    }

    fn set_velocity(&mut self, velocity: Vector2<f32>) {
    	self.velocity = velocity;
    }

    fn velocity(&self) -> Vector2<f32> {
    	self.velocity
    }

    fn set_hit_box(&mut self, hit_box: Vector2<f32>) {
    	self.hit_box = hit_box;
    }

    fn hit_box(&self) -> Vector2<f32> {
    	self.hit_box
    }
}
