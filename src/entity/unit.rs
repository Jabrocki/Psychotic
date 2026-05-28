use crate::entities::Entity;

pub trait UnitEntity: Entity {
    fn set_hp(&mut self, hp: i32);
    fn hp(&self) -> i32;

    fn set_max_hp(&mut self, max_hp: i32);
    fn max_hp(&self) -> i32;

    fn atk(&self) -> i32;
    fn df(&self) -> i32;
}
