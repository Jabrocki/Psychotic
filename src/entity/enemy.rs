use nalgebra::{Rotation2, Vector2};
use rand::seq::IndexedRandom;

use crate::{entities::Entity, entity::unit::UnitEntity, movement::Moveable, player::Player};

pub struct Enemy{
	position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    hit_box: Vector2<f32>,

    hp: i32,
    max_hp: i32,
    atk: i32,
    df: i32,

    move_ticks: i32,
    chase_dir: Vector2<f32>
}

impl Enemy {
	pub fn new(position: Vector2<f32>) -> Self{
		Self{
			position,
			velocity: Vector2::new(0., 0.),
			acceleration: Vector2::new(0., 0.),
			hit_box: Vector2::new(0.1, 0.1),

			hp: 10,
			max_hp: 10,
			atk: 1,
			df: 1,

			move_ticks: 0,
			chase_dir: Vector2::new(0., 0.)
		}
	}

	fn update_chase_dir(&mut self, world: &mut crate::world::World){
		if self.move_ticks == 0{
			let p = world.player.unwrap();
			let p: &mut Player = unsafe {&mut *p};
			let player_pos = p.position();
			let dir = (player_pos - self.position).normalize();

			if dir.norm() > 10.{
				return;
			}

			let angle = (vec![-1., 0., 1.].choose(&mut world.random)).unwrap() * 20. / 3.14;
			let rotation_matrix: Rotation2<f32> = Rotation2::new(angle);

			self.chase_dir = rotation_matrix * dir;
			self.move_ticks+=75;
		}
		self.move_ticks-=1;

		self.acceleration = Vector2::new(self.chase_dir.x*20., self.chase_dir.y*20.);
	}
}

impl Entity for Enemy {
    fn position(&self) -> Vector2<f32> {
        self.position
    }

    fn set_position(&mut self, pos: nalgebra::Vector2<f32>) {
    	self.position = pos;
    }

    fn update(&mut self, world: &mut crate::world::World, delta_time: f32) {
    	self.update_chase_dir(world);
    	let hit = self.simulate_movement(world, delta_time);

     	if hit{
      		let angle = (vec![-1., 1.].choose(&mut world.random)).unwrap() * 3.14 / 2.;
	        let rotation_matrix: Rotation2<f32> = Rotation2::new(angle);
      		self.chase_dir = rotation_matrix * self.chase_dir;
      	}
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
