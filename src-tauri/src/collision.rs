use serde::{Deserialize, Serialize};
use crate::physics::{Ball, Vec2};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wall {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hole {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub id: u32,
}

const RESTITUTION: f32 = 0.7;

impl Wall {
    /// If the ball overlaps this wall, returns (normal pointing at the ball, overlap depth).
    pub fn contact(&self, ball: &Ball) -> Option<(Vec2, f32)> {
        let dx = self.x2 - self.x1;
        let dy = self.y2 - self.y1;
        let len_sq = dx * dx + dy * dy;
        if len_sq == 0.0 {
            return None;
        }

        let t = (((ball.position.x - self.x1) * dx + (ball.position.y - self.y1) * dy) / len_sq).clamp(0.0, 1.0);
        let closest = Vec2::new(self.x1 + t * dx, self.y1 + t * dy);
        let offset = Vec2::new(ball.position.x - closest.x, ball.position.y - closest.y);
        let dist = offset.magnitude();

        if dist >= ball.radius {
            return None;
        }
        let normal = if dist > 1e-6 {
            offset.normalize()
        } else {
            // Ball centre sits exactly on the wall: push out perpendicular to it.
            Vec2::new(-dy, dx).normalize()
        };
        Some((normal, ball.radius - dist))
    }
}

/// Push the ball out of a wall and reflect the velocity component going into it.
pub fn resolve_wall(ball: &mut Ball, normal: Vec2, depth: f32) {
    ball.position = ball.position.add(normal.scale(depth));
    let vn = ball.velocity.dot(normal);
    if vn < 0.0 {
        ball.velocity = ball.velocity.add(normal.scale(-(1.0 + RESTITUTION) * vn));
    }
}

pub fn check_hole_collision(ball: &Ball, hole: &Hole) -> bool {
    let dx = ball.position.x - hole.x;
    let dy = ball.position.y - hole.y;
    (dx * dx + dy * dy).sqrt() < hole.radius
}
