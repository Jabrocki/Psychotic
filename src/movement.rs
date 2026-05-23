use nalgebra::Vector2;

use crate::{entities::Entity, world::World};

pub trait Moveable : Entity{
	fn set_acceleration(&mut self, acceleration: Vector2<f32>);
	fn acceleration(&self) -> Vector2<f32>;

	fn set_velocity(&mut self, velocity: Vector2<f32>);
	fn velocity(&self) -> Vector2<f32>;

	fn set_hit_box(&mut self, velocity: Vector2<f32>);
	fn hit_box(&self) -> Vector2<f32>;

	fn simulate_movement(&mut self, world: &mut World, delta_time: f32) {
		self.set_velocity(self.velocity() + self.acceleration() * delta_time);
        let new_position_x = self.position().x + self.velocity().x * delta_time;
        if !world.wall_collsion(Vector2::new(new_position_x, self.position().y), self.hit_box()) {
        	let set_pos = Vector2::new(new_position_x, self.position().y);
         	let set_vel = Vector2::new(self.velocity().x * 0.8, self.velocity().y);

            self.set_position(set_pos);
            self.set_velocity(set_vel);
        } else {
        	let set_vel = Vector2::new(0., self.velocity().y);
         	self.set_velocity(set_vel);
        }

        let new_position_y = self.position().y + self.velocity().y * delta_time;
        if !world.wall_collsion(Vector2::new(self.position().x, new_position_y), self.hit_box()) {
       		let set_pos = Vector2::new(self.position().x, new_position_y);
        	let set_vel = Vector2::new(self.velocity().x, self.velocity().y * 0.8);

         	self.set_position(set_pos);
          	self.set_velocity(set_vel);
        } else {
       		let set_vel = Vector2::new(self.velocity().x, 0.);
        	self.set_velocity(set_vel);
        }
        self.set_acceleration(Vector2::new(0., 0.));
	}
}
