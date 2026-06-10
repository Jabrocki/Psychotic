use std::rc::Rc;

use crate::level::room_file::RoomFile;
use nalgebra::Vector2;
use uuid::Uuid;

pub struct Room {
    id: Uuid,
    left_top_corner: Vector2<usize>,
    pub file: Rc<RoomFile>,
}

impl Room {
    pub fn new(file: Rc<RoomFile>, left_top_corner: Vector2<usize>) -> Self {
        Self {
            id: Uuid::now_v7(),
            left_top_corner,
            file,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_left_top_corner(&self) -> Vector2<usize> {
        self.left_top_corner
    }
}
