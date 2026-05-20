use nalgebra::Vector2;
use uuid::Uuid;

#[derive(Clone)]
pub struct Edge {
    from_id: Uuid,
    to_id: Uuid,
    from: Vector2<usize>,
    to: Vector2<usize>,
    weight: f32,
}

#[derive(Clone)]
pub struct Door {
    pub room_id: Uuid,
    pub position: Vector2<usize>,
}

impl Edge {
    pub fn new(from: Door, to: Door) -> Self {
        let weigth = (to.position.cast::<f32>() - from.position.cast::<f32>()).norm();
        Self {
            from_id: from.room_id,
            to_id: to.room_id,
            from: from.position,
            to: to.position,
            weight: weigth,
        }
    }

    pub fn get_from(&self) -> Vector2<usize> {
        self.from
    }

    pub fn get_from_id(&self) -> Uuid {
        self.from_id
    }

    pub fn get_to_id(&self) -> Uuid {
        self.to_id
    }

    pub fn get_to(&self) -> Vector2<usize> {
        self.to
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }
}
