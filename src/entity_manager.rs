use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::ptr::copy;

use nalgebra::Vector2;
use std::mem::{self, ManuallyDrop, transmute};
use std::alloc::{alloc, dealloc, Layout};

pub trait Entity{
	fn position(&self) -> Vector2<f32>;
	fn set_position(&mut self, pos: Vector2<f32>);

	fn update(&mut self);

	fn sprite_id(&self) -> u8;
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct EntityPtr{
	vtable: *mut dyn Entity,
	data: *mut u8
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
		let no_drop_entity : TempContainer<T> = TempContainer::new(entity);

		let data: *mut u8 = unsafe{ alloc(Layout::new::<T>()) };
		let data_t: *mut T = data as *mut T;
		unsafe{
			copy(&no_drop_entity.val, data_t, size_of::<T>());
		}
		let vtable: *mut dyn Entity = data_t;

		EntityPtr{
			vtable,
			data,
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

    fn update(&mut self) {

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
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, position: Vector2<f32>, sprite_id: u8) {
        self.entities.push(EntityPtr::from::<Dummy>(Dummy::new (
            position,
            sprite_id,
        )));
    }

    pub fn get_entities(&self) -> &Vec<EntityPtr> {
        &self.entities
    }
}
