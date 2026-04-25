use nalgebra::Vector2;

pub struct Player {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,

    yaw: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(32.0, 32.0),
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            yaw: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
        self.acceleration = Vector2::new(0.0, 0.0);
        self.velocity *= 0.99; // Friction
        println!("Position: ({}, {})", self.position.x, self.position.y);
        println!("Velocity: ({}, {})", self.velocity.x, self.velocity.y);
        println!(
            "Acceleration: ({}, {})",
            self.acceleration.x, self.acceleration.y
        );
    }

    pub fn left(&mut self) {
        self.acceleration.x -= 5.0;
    }

    pub fn right(&mut self) {
        self.acceleration.x += 5.0;
    }

    pub fn up(&mut self) {
        self.acceleration.y -= 5.0;
    }

    pub fn down(&mut self) {
        self.acceleration.y += 5.0;
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn yaw(&self) -> f32 {
        self.yaw
    }
}
