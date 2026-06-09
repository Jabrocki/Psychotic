use nalgebra::{Rotation2, Vector2};
use rand::seq::IndexedRandom;

use crate::{
    animation::AnimationState, entities::{Entity, entity_id}, entity::unit::UnitEntity, movement::Moveable,
    pathfinder::find_path, player::Player, shoot::raycast2d, world::World,
};

#[derive(Debug)]
enum EnemyState {
    ChaseState,
    PathfindingState { path: Vec<Vector2<i32>> },
    Idle,
    Inactive,
}

pub struct Enemy {
	id: entity_id,

    position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    hit_box: Vector2<f32>,

    hp: i32,
    max_hp: i32,
    atk: i32,
    df: i32,

    state: EnemyState,
    direction: Vector2<f32>,
    move_ticks: i32,

    no_bounce: bool,

    animation_state: AnimationState,

    fire_delay: f32,
}

impl Enemy {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
        	id: 0,
            position,
            velocity: Vector2::new(0., 0.),
            acceleration: Vector2::new(0., 0.),
            hit_box: Vector2::new(0.4, 0.4),

            hp: 10,
            max_hp: 10,
            atk: 1,
            df: 1,

            state: EnemyState::Inactive,
            move_ticks: 0,
            direction: Vector2::new(0., 0.),

            no_bounce: false,

            animation_state: AnimationState::new(11),
            fire_delay: 0.,
        }
    }

    fn update_chase(&mut self, world: &mut crate::world::World) -> EnemyState {
        self.no_bounce = false;
        if self.move_ticks == 0 {
            let p = world.player.unwrap();
            let p: &mut Player = unsafe { &mut *p };
            let player_pos = p.position();

            let dir = player_pos - self.position;

            if dir.norm() > 10. {
                return EnemyState::PathfindingState { path: vec![] };
            }

            let dir = dir.normalize();

            let angle = (vec![-1., 0., 1.].choose(&mut world.random)).unwrap() * 20. / 3.14;
            let rotation_matrix: Rotation2<f32> = Rotation2::new(angle);

            self.direction = rotation_matrix * dir;
            self.move_ticks += 75;
        }
        self.move_ticks -= 1;

        self.acceleration = Vector2::new(self.direction.x * 20., self.direction.y * 20.);
        EnemyState::ChaseState
    }

    fn update_pathfinding(
        &mut self,
        world: &mut crate::world::World,
        mut path: Vec<Vector2<i32>>,
    ) -> EnemyState {
        self.no_bounce = true;

        let p = world.player.unwrap();
        let p: &mut Player = unsafe { &mut *p };
        let player_pos = p.position();

        if (self.position - player_pos).norm() <= 2. {
            return EnemyState::ChaseState;
        }

        if path.len() == 0 {
            path = find_path(
                player_pos.x as i32,
                player_pos.y as i32,
                self.position.x as i32,
                self.position.y as i32,
                world,
            )
            .unwrap_or_default();
            path.reverse();
        } else {
            if self.move_ticks <= 0 {
                let target = path.pop().unwrap();
                let target = Vector2::new(target.x as f32 + 0.5, target.y as f32 + 0.5);
                let dir = (target - self.position).normalize();

                self.direction = dir;
                self.move_ticks += 75;
            }
        }
        self.move_ticks -= 1;
        self.acceleration = Vector2::new(self.direction.x * 20., self.direction.y * 20.);

        EnemyState::PathfindingState { path }
    }

    fn try_shooting(&mut self, world: &mut crate::world::World) {
        if self.fire_delay <= 0. {
            let p = world.player.unwrap();
            let p: &mut Player = unsafe { &mut *p };
            let player_pos = p.position();
            let dir = (player_pos - self.position).normalize();

            if dir.norm() > 10. {
                return;
            }

            if (player_pos - self.position).norm() < 0.2
                || (raycast2d(world, self.position, dir) - self.position).norm()
                    > (player_pos - self.position).norm()
            {
                // we can shoot
                self.animation_state.set_frame(12, 1.);
                self.fire_delay += 2.;

                p.set_hp(p.hp()-self.atk);
            }
        }
    }

    fn seek_player(&mut self, world: &World) -> EnemyState {
        let p = world.player.unwrap();
        let p: &mut Player = unsafe { &mut *p };
        let player_pos = p.position();

        if (self.position - player_pos).norm() <= 5. {
            EnemyState::ChaseState
        } else {
            EnemyState::Inactive
        }
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
        let mut old_state = EnemyState::Idle;
        std::mem::swap(&mut self.state, &mut old_state);

        self.state = match old_state {
            EnemyState::ChaseState => self.update_chase(world),
            EnemyState::PathfindingState { path } => self.update_pathfinding(world, path),
            EnemyState::Inactive => self.seek_player(world),
            _ => EnemyState::ChaseState,
        };
        let hit = self.simulate_movement(world, delta_time);

        if hit && !self.no_bounce {
            let angle = (vec![-1., 1.].choose(&mut world.random)).unwrap() * 3.14 / 2.;
            let rotation_matrix: Rotation2<f32> = Rotation2::new(angle);
            self.direction = rotation_matrix * self.direction;
        }

        self.fire_delay -= delta_time;
        self.try_shooting(world);
        self.animation_state.update(delta_time);

        if self.hp <= 0 {
        	world.remove_entity(self.id);
        }
    }

    fn sprite_id(&self) -> Option<u8> {
        Some(self.animation_state.current_state)
    }

    fn bounding_box(&self) -> Vector2<f32> {
        self.hit_box
    }

    fn set_id(&mut self, id: entity_id){
    	self.id = id;
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

impl Moveable for Enemy {
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
