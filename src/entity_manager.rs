use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::ptr::copy;

use nalgebra::Vector2;
use std::mem::{self, ManuallyDrop, transmute};
use std::alloc::{alloc, dealloc, Layout};

use crate::entity_manager;

type entity_id = u32;

pub trait Entity{
	fn position(&self) -> Vector2<f32>;
	fn set_position(&mut self, pos: Vector2<f32>);

	fn update(&mut self, entities: &mut EntityManager);

	fn sprite_id(&self) -> u8;
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct EntityPtr{
	id: entity_id,
	vtable: *mut dyn Entity,
	dynamic: *mut dyn Any,
	data: *mut u8,

	layout: Layout
}

impl DerefMut for EntityPtr{

    fn deref_mut(&mut self) -> &mut Self::Target {
    	unsafe { transmute(self.vtable) }
    }
}

impl Deref for EntityPtr{
    type Target = dyn Entity;

    fn deref(&self) -> &Self::Target {
    	unsafe { transmute(self.vtable) }
    }
}

struct TempContainer<T>{
	val: T,
}

impl<T> Drop for TempContainer<T>{
    fn drop(&mut self) {

    }
}

impl<T> TempContainer<T>{
	fn new(m: T) -> TempContainer<T>{
		TempContainer { val: m }
	}
}

impl EntityPtr{
	fn from<T: Entity + 'static>(entity: T) -> EntityPtr{
		let no_drop_entity : TempContainer<T> = TempContainer::new(entity); // hijack entity in order not to call drop

		let layout = Layout::new::<T>();

		let data: *mut u8 = unsafe{ alloc(layout) }; // malloc
		let data_t: *mut T = data as *mut T;
		unsafe{
			copy(&no_drop_entity.val, data_t, size_of::<T>());
		}
		let vtable: *mut dyn Entity = data_t;
		let dynamic: *mut dyn Any = data_t; // for downcasting

		let id = rand::random();

		EntityPtr{
			id,
			vtable,
			data,
			dynamic,
			layout
		}
	}

	fn id(&self) -> entity_id{
		self.id
	}

	fn destroy(&mut self) {
		unsafe{
			self.vtable.drop_in_place();
			dealloc(self.data, self.layout);
		}
	}
}

struct Dummy{
	pos: Vector2<f32>,
	sprite_id: u8
}

impl Entity for Dummy{
    fn position(&self) -> Vector2<f32> {
        self.pos
    }

    fn set_position(&mut self, pos: Vector2<f32>) {
        self.pos = pos;
    }

    fn update(&mut self, entities: &mut EntityManager) {

    }

    fn sprite_id(&self) -> u8 {
    	self.sprite_id
    }
}

impl Dummy{
	fn new(pos: Vector2<f32>, sprite_id: u8) -> Self{
		Dummy {pos, sprite_id}
	}
}

pub struct EntityManager {
    entities: Vec<EntityPtr>,
    stage_added: Vec<EntityPtr>,
    stage_removed: Vec<entity_id>
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            stage_added: Vec::new(),
            stage_removed: Vec::new()
        }
    }

    // entities should not call this
    pub fn add_entity_dummy(&mut self, position: Vector2<f32>, sprite_id: u8) {
        self.entities.push(EntityPtr::from::<Dummy>(Dummy::new (
            position,
            sprite_id,
        )));
    }

    pub fn stage_add_entity(&mut self, entity: EntityPtr){
    	self.stage_added.push(entity);
    }

    pub fn get_entities(&self) -> &Vec<EntityPtr> {
        &self.entities
    }

    pub fn remove_entity(&mut self, id: entity_id){
    	self.stage_removed.push(id);
    }

    pub fn update(&mut self) {
       	// remove all entities to be removed
        let staged_destroy = self.entities.iter()
        	.filter(|&ent| self.stage_removed.contains(&ent.id));
        for &ent in staged_destroy{
        	let mut e2 = ent;
         	e2.destroy();
        }

        self.entities.retain(|&ent| !self.stage_removed.contains(&ent.id));
        self.stage_removed.clear();

        // add entities to be added
        for &ent in &self.stage_added{
        	self.entities.push(ent);
        }
        self.stage_added.clear();

    	let self2 = self as *mut Self;
     	let self2: &mut Self = unsafe {transmute(self2)}; // ignore guarantees about two mutable refs

    	for ent in &mut self.entities{
     		ent.update(self2);
     	}

    }
}
