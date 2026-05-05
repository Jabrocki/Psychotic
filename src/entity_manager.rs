use std::any::Any;
use std::ops::{Deref, DerefMut};

use nalgebra::Vector2;
use std::alloc::{Layout, alloc, dealloc};
use std::mem::{self, ManuallyDrop, transmute};

use crate::world::World;

pub type entity_id = u32;
pub trait Entity {
    fn position(&self) -> Vector2<f32>;
    fn set_position(&mut self, pos: Vector2<f32>);

    fn update(&mut self, entities: &mut World, delta_time: f32);

    fn sprite_id(&self) -> Option<u8>;
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct EntityPtr {
    id: entity_id,
    vtable: *mut dyn Entity,
    dynamic: *mut dyn Any,
    data: *mut u8,

    layout: Layout,
}

impl DerefMut for EntityPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.vtable) }
    }
}

impl Deref for EntityPtr {
    type Target = dyn Entity;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.vtable) }
    }
}

impl EntityPtr {
    pub fn new<T: Entity + 'static>(entity: T) -> EntityPtr {
        let layout = Layout::new::<T>();

        let data: *mut u8 = unsafe { alloc(layout) }; // malloc
        let data_t: *mut T = data as *mut T;

        unsafe {
            std::ptr::write(data_t, entity);
        }

        let vtable: *mut dyn Entity = data_t;
        let dynamic: *mut dyn Any = data_t; // for downcasting

        let id = rand::random();

        EntityPtr {
            id,
            vtable,
            data,
            dynamic,
            layout,
        }
    }

    pub fn id(&self) -> entity_id {
        self.id
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.vtable.drop_in_place();
            dealloc(self.data, self.layout);
        }
    }

    pub fn direct<T>(&mut self) -> &'static mut T {
        unsafe { &mut *(self.data as *mut T) }
    }
}

pub struct Dummy {
    pos: Vector2<f32>,
    sprite_id: u8,
}

impl Entity for Dummy {
    fn position(&self) -> Vector2<f32> {
        self.pos
    }

    fn set_position(&mut self, pos: Vector2<f32>) {
        self.pos = pos;
    }

    fn update(&mut self, entities: &mut World, delta_time: f32) {}

    fn sprite_id(&self) -> Option<u8> {
        Some(self.sprite_id)
    }
}

impl Dummy {
    pub fn new(pos: Vector2<f32>, sprite_id: u8) -> Self {
        Dummy { pos, sprite_id }
    }
}
