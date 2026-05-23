use nalgebra::Vector2;

use crate::{entities::Entity, entity::unit::UnitEntity};

pub struct Enemy{

}

impl Entity for Enemy {
    fn position(&self) -> Vector2<f32> {
        todo!()
    }

    fn set_position(&mut self, pos: nalgebra::Vector2<f32>) {
        todo!()
    }

    fn update(&mut self, entities: &mut crate::world::World, delta_time: f32) {
        todo!()
    }

    fn sprite_id(&self) -> Option<u8> {
        todo!()
    }
}

impl UnitEntity for Enemy {
    fn set_hp(&mut self, hp: i32) {
        todo!()
    }

    fn hp(&self) -> i32 {
        todo!()
    }

    fn set_max_hp(&mut self, max_hp: i32) {
        todo!()
    }

    fn max_hp(&self) -> i32 {
        todo!()
    }

    fn atk(&self) -> i32 {
        todo!()
    }

    fn df(&self) -> i32 {
        todo!()
    }
}
