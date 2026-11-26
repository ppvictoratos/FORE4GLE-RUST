use serde::{Deserialize, Serialize};
use crate::collision::{check_hole_collision, resolve_wall};
use crate::hole::HoleLayout;
use crate::physics::{Ball, Vec2};

const STRIKE_SPEED: f32 = 500.0;
/// The ball only drops in the cup if it's moving slower than this.
const CUP_SPEED: f32 = 260.0;
const SUBSTEPS: u32 = 4;
const CANVAS_W: f32 = 750.0;
const CANVAS_H: f32 = 700.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub ball: Ball,
    pub hole_layout: HoleLayout,
    pub ball_sunk: Option<u32>,
    pub hole_index: usize,
    pub hole_count: usize,
    pub strokes: u32,
    /// Strokes per hole, None until that hole is holed.
    pub scores: Vec<Option<u32>>,
    pub power: f32,
    pub angle: f32,
    /// The whole course; not sent to the frontend on every frame.
    #[serde(skip)]
    pub course: Vec<HoleLayout>,
}

impl GameState {
    pub fn new(course: Vec<HoleLayout>) -> Self {
        let count = course.len();
        let mut game = GameState {
            ball: Ball::new(0.0, 0.0),
            hole_layout: course[0].clone(),
            ball_sunk: None,
            hole_index: 0,
            hole_count: count,
            strokes: 0,
            scores: vec![None; count],
            power: 0.0,
            angle: 0.0,
            course,
        };
        game.load_hole(0);
        game
    }

    fn load_hole(&mut self, index: usize) {
        self.hole_index = index;
        self.hole_layout = self.course[index].clone();
        self.ball = Ball::new(self.hole_layout.start_pos.x, self.hole_layout.start_pos.y);
        self.ball_sunk = None;
        self.strokes = 0;
    }

    pub fn update(&mut self, dt: f32) {
        if self.ball_sunk.is_some() {
            return;
        }

        for _ in 0..SUBSTEPS {
            self.ball.update(dt / SUBSTEPS as f32);

            for wall in &self.hole_layout.walls {
                if let Some((normal, depth)) = wall.contact(&self.ball) {
                    resolve_wall(&mut self.ball, normal, depth);
                }
            }

            let r = self.ball.radius;
            if self.ball.position.x < r { self.ball.position.x = r; self.ball.velocity.x = self.ball.velocity.x.abs() * 0.7; }
            if self.ball.position.x > CANVAS_W - r { self.ball.position.x = CANVAS_W - r; self.ball.velocity.x = -self.ball.velocity.x.abs() * 0.7; }
            if self.ball.position.y < r { self.ball.position.y = r; self.ball.velocity.y = self.ball.velocity.y.abs() * 0.7; }
            if self.ball.position.y > CANVAS_H - r { self.ball.position.y = CANVAS_H - r; self.ball.velocity.y = -self.ball.velocity.y.abs() * 0.7; }

            let speed = self.ball.velocity.magnitude();
            let sunk_in = self
                .hole_layout
                .holes
                .iter()
                .find(|h| check_hole_collision(&self.ball, h) && speed < CUP_SPEED)
                .cloned();
            if let Some(hole) = sunk_in {
                self.ball.position = Vec2::new(hole.x, hole.y);
                self.ball.velocity = Vec2::zero();
                self.ball_sunk = Some(hole.id);
                self.scores[self.hole_index] = Some(self.strokes);
                return;
            }
        }
    }

    /// Returns false (and does nothing) if the ball is still rolling or already holed.
    pub fn strike(&mut self, angle: f32, power: f32) -> bool {
        if self.ball_sunk.is_some() || self.ball.velocity.magnitude() > 0.0 {
            return false;
        }
        let power = power.clamp(0.0, 1.0);
        self.angle = angle;
        self.power = power;
        let speed = power * STRIKE_SPEED;
        self.ball.velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        self.strokes += 1;
        true
    }

    /// Restart the current hole (strokes go back to zero).
    pub fn reset(&mut self) {
        let index = self.hole_index;
        self.load_hole(index);
    }

    /// After holing out: go to the next hole, or start a fresh course after the last one.
    pub fn next_hole(&mut self) {
        if self.ball_sunk.is_none() {
            return;
        }
        if self.hole_index + 1 < self.hole_count {
            self.load_hole(self.hole_index + 1);
        } else {
            self.scores = vec![None; self.hole_count];
            self.load_hole(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hole::course;

    /// Breadth-first search over resting spots, one level per stroke. Every resting spot
    /// (one per 20px cell) is expanded with a grid of angles and powers. Stops at the first
    /// shot that holes out, so this proves each hole can be finished with the real physics.
    fn strokes_to_hole_out(index: usize) -> Option<u32> {
        const MAX_STROKES: u32 = 12;
        const MAX_EXPANSIONS_PER_LEVEL: usize = 400;

        let mut g = GameState::new(course());
        g.load_hole(index);
        let start = Vec2::new(g.ball.position.x, g.ball.position.y);
        let mut seen = std::collections::HashSet::new();
        let mut level = vec![start];

        for stroke in 1..=MAX_STROKES {
            let mut next = Vec::new();
            for pos in level.iter().take(MAX_EXPANSIONS_PER_LEVEL) {
                for deg in (0..360).step_by(6) {
                    for p in [0.4f32, 0.7, 1.0] {
                        g.ball = Ball::new(pos.x, pos.y);
                        g.ball_sunk = None;
                        assert!(g.strike((deg as f32).to_radians(), p));
                        for _ in 0..1200 {
                            g.update(1.0 / 60.0);
                            if g.ball_sunk.is_some() || g.ball.velocity.magnitude() == 0.0 {
                                break;
                            }
                        }
                        if g.ball_sunk.is_some() {
                            return Some(stroke);
                        }
                        let rest = Vec2::new(g.ball.position.x, g.ball.position.y);
                        let cell = ((rest.x / 20.0) as i32, (rest.y / 20.0) as i32);
                        if seen.insert(cell) {
                            next.push(rest);
                        }
                    }
                }
            }
            if next.is_empty() {
                return None;
            }
            level = next;
        }
        None
    }

    #[test]
    fn course_has_18_playable_holes() {
        let holes = course();
        assert_eq!(holes.len(), 18);
        for i in 0..holes.len() {
            let strokes = strokes_to_hole_out(i);
            println!("hole {:>2} {:<18} par {} -> {:?}", i + 1, holes[i].name, holes[i].par, strokes);
            assert!(strokes.is_some(), "hole {} ({}) is not reachable", i + 1, holes[i].name);
        }
    }

    #[test]
    fn next_hole_cycles_the_course() {
        let mut g = GameState::new(course());
        g.ball_sunk = Some(1);
        g.next_hole();
        assert_eq!(g.hole_index, 1);
        g.load_hole(17);
        g.ball_sunk = Some(1);
        g.next_hole();
        assert_eq!(g.hole_index, 0);
        assert!(g.scores.iter().all(|s| s.is_none()));
    }
}
