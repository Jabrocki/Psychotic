use nalgebra::{Rotation2, Vector2};

use crate::{
    entities::Entity,
    entity::{enemy::Enemy, unit::UnitEntity},
    movement::Moveable,
    shoot::{raycast2d, raycast2d_entities_except_player},
    world::World,
};

pub struct Player {
    position: Vector2<f32>,
    direction: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    hit_box: Vector2<f32>,

    shooting_requested: bool,

    hp: i32,
    max_hp: i32,
    atk: i32,
    df: i32,

    damage_effect: i32
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(1.5, 1.5),
            direction: Vector2::new(-1.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            hit_box: Vector2::new(0.1, 0.1),

            hp: 60,
            max_hp: 60,
            atk: 2,
            df: 0,

            shooting_requested: false,

            damage_effect: 0,
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
    	if self.hp <= 0 {return;}

        self.acceleration += self.direction * 50.0;
    }

    pub fn down(&mut self) {
    	if self.hp <= 0 {return;}

        self.acceleration -= self.direction * 50.0;
    }

    pub fn left(&mut self) {
    	if self.hp <= 0 {return;}

        let left_direction = Vector2::new(self.direction.y, -self.direction.x);
        self.acceleration += left_direction * 50.0;
    }

    pub fn right(&mut self) {
    	if self.hp <= 0 {return;}

        let right_direction = Vector2::new(-self.direction.y, self.direction.x);
        self.acceleration += right_direction * 50.0;
    }

    pub fn rotate(&mut self, angle: f32) {
    	if self.hp <= 0 {return;}

        let rotation_matrix: Rotation2<f32> = Rotation2::new(angle);
        self.direction = rotation_matrix * self.direction;
    }

    pub fn shoot(&mut self) {
    	if self.hp <= 0 {return;}

        self.shooting_requested = true;
    }

    pub fn set_damage_effect(&mut self, effect: i32){
    	self.damage_effect = effect;
    }

    pub fn get_damage_effect(&self) -> i32{
    	self.damage_effect
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
        self.simulate_movement(world, delta_time);

        if self.shooting_requested {
            let raycast_entity_res =
                raycast2d_entities_except_player(world, self.position, self.direction);
            let raycast_wall = raycast2d(world, self.position, self.direction);

            //println!("{:?} {:?}", raycast_entity_res, raycast_wall);

            match raycast_entity_res {
                Some((entity_id, pos)) => {
                    let diff1 = raycast_wall - self.position;
                    let diff_entity = pos - self.position;
                    if diff_entity.norm() < diff1.norm() {
                        let extracted = world
                            .get_entities()
                            .iter()
                            .filter(|e| e.id() == entity_id)
                            .next();
                        match extracted {
                            Some(&target) => {
                                let mut target = target;
                                let unit_target: Option<&mut Enemy> = target.any().downcast_mut();

                                match unit_target {
                                    Some(enemy_ref) => {
                                        //world.remove_entity(target.id());
                                        enemy_ref.set_hp(enemy_ref.hp()-self.atk);
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }
            self.shooting_requested = false;
        }

        if self.damage_effect > 0 {
       		self.damage_effect -= 3;
        }
    }

    fn sprite_id(&self) -> Option<u8> {
        None
    }

    fn bounding_box(&self) -> Vector2<f32> {
        self.hit_box
    }
}

impl UnitEntity for Player {
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

impl Moveable for Player {
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
}
