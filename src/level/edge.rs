use nalgebra::Vector2;

#[derive(Clone)]
pub struct Edge {
    from: Vector2<usize>,
    to: Vector2<usize>,
    weight: f32,
}

impl Edge {
    pub fn new(from: Vector2<usize>, to: Vector2<usize>) -> Self {
        let weight = (to.cast::<f32>() - from.cast::<f32>()).norm();
        Self { from, to, weight }
    }

    pub fn get_from(&self) -> Vector2<usize> {
        self.from
    }

    pub fn get_to(&self) -> Vector2<usize> {
        self.to
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }
}
