use nalgebra::Vector2;

pub struct Entity {
    position: Vector2<f32>,
    sprite_id: u32,
}

impl Entity {
    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn sprite_id(&self) -> u32 {
        self.sprite_id
    }
}

pub struct EntityManager {
    entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, position: Vector2<f32>, sprite_id: u32) {
        self.entities.push(Entity {
            position,
            sprite_id,
        });
    }

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}
