use nalgebra::Vector2;
use rand::SeedableRng;

use crate::{entities::{Dummy, Entity, EntityPtr}, player::Player};

pub struct World {
    pub width: usize,
    pub height: usize,
    pub wall_map: Vec<Vec<i16>>,
    pub ceil_map: Vec<Vec<i16>>,
    pub floor_map: Vec<Vec<i16>>,
    entities: Vec<EntityPtr>,
    stage_added: Vec<EntityPtr>,
    stage_removed: Vec<entity_id>,
    pub random: rand::rngs::SmallRng,

    pub player: Option<*mut Player>,
}

pub type entity_id = u32;

unsafe impl Send for World {

}

unsafe impl Sync for World {

}

impl World {
    pub fn new(wall_map: Vec<Vec<i16>>, ceil_map: Vec<Vec<i16>>, floor_map: Vec<Vec<i16>>) -> Self {
        let height = wall_map.len();
        let width = if height > 0 { wall_map[0].len() } else { 0 };
        Self {
            width,
            height,
            wall_map,
            ceil_map,
            floor_map,
            entities: Vec::new(),
            stage_added: Vec::new(),
            stage_removed: Vec::new(),
            player: None,
            random: rand::rngs::SmallRng::seed_from_u64(128 as u64)
        }
    }

    pub fn get_wall_tile(&self, x: usize, y: usize) -> i16 {
        if y < self.height && x < self.width {
            self.wall_map[y][x]
        } else {
            1
        }
    }

    pub fn get_ceil_tile(&self, x: usize, y: usize) -> i16 {
        if y < self.height && x < self.width {
            self.ceil_map[y][x]
        } else {
            0
        }
    }

    pub fn get_floor_tile(&self, x: usize, y: usize) -> i16 {
        if y < self.height && x < self.width {
            self.floor_map[y][x]
        } else {
            0
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

// Wordl impl for Entities
impl World {
    // entities should not call this
    pub fn add_entity_dummy(&mut self, position: Vector2<f32>, sprite_id: u8) {
        self.entities
            .push(EntityPtr::new::<Dummy>(Dummy::new(position, sprite_id)));
    }

    pub fn stage_add_entity(&mut self, entity: EntityPtr) {
        self.stage_added.push(entity);
    }

    pub fn force_add_entity(&mut self, entity: EntityPtr) {
        self.entities.push(entity);
    }

    pub fn stage_add_entity_direct<T: Entity + 'static>(&mut self, entity: T) -> (EntityPtr, &mut T){
    	let mut eptr = EntityPtr::new::<T>(entity);

    	self.stage_add_entity(eptr);

     	(eptr, eptr.direct())
    }

    pub fn force_add_entity_direct<T: Entity + 'static>(&mut self, entity: T) -> (EntityPtr, &mut T){
    	let mut eptr = EntityPtr::new::<T>(entity);

    	self.force_add_entity(eptr);

     	(eptr, eptr.direct())
    }

    pub fn get_entities(&self) -> &Vec<EntityPtr> {
        &self.entities
    }

    pub fn remove_entity(&mut self, id: entity_id) {
        self.stage_removed.push(id);
    }

    pub fn update(&mut self, delta_time: f32) {
        // remove all entities to be removed
        let staged_destroy = self
            .entities
            .iter()
            .filter(|&ent| self.stage_removed.contains(&ent.id()));
        for &ent in staged_destroy {
            let mut e2 = ent;
            e2.destroy();
        }

        self.entities
            .retain(|&ent| !self.stage_removed.contains(&ent.id()));
        self.stage_removed.clear();

        // add entities to be added
        for &ent in &self.stage_added {
            self.entities.push(ent);
        }
        self.stage_added.clear();

        let self2 = self as *mut Self;
        let self2: &mut Self = unsafe { &mut *self2 }; // ignore guarantees about two mutable refs

        for ent in &mut self.entities {
            ent.update(self2, delta_time);
        }
    }

    pub fn wall_collsion(&self, pos: Vector2<f32>, hit_rect: Vector2<f32>) -> bool {
        let left = (pos.x - hit_rect.x) as usize;
        let right = (pos.x + hit_rect.x) as usize;
        let top = (pos.y - hit_rect.y) as usize;
        let bottom = (pos.y + hit_rect.y) as usize;

        self.get_wall_tile(left, top) != 0
            || self.get_wall_tile(right, top) != 0
            || self.get_wall_tile(left, bottom) != 0
            || self.get_wall_tile(right, bottom) != 0
    }
}
