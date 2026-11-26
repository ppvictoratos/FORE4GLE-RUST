use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }

    pub fn scale(self, scalar: f32) -> Vec2 {
        Vec2 { x: self.x * scalar, y: self.y * scalar }
    }

    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(self) -> Vec2 {
        let mag = self.magnitude();
        if mag > 0.0 { Vec2 { x: self.x / mag, y: self.y / mag } } else { Vec2::zero() }
    }

    pub fn dot(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Ball { position: Vec2::new(x, y), velocity: Vec2::zero(), radius: 8.0 }
    }

    /// Top-down rolling: no gravity, just friction, and a hard stop once it's crawling.
    pub fn update(&mut self, dt: f32) {
        const FRICTION_PER_60HZ_FRAME: f32 = 0.985;
        const STOP_SPEED: f32 = 6.0;

        self.position = self.position.add(self.velocity.scale(dt));
        self.velocity = self.velocity.scale(FRICTION_PER_60HZ_FRAME.powf(dt * 60.0));
        if self.velocity.magnitude() < STOP_SPEED {
            self.velocity = Vec2::zero();
        }
    }
}
