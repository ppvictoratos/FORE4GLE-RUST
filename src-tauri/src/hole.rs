use serde::{Deserialize, Serialize};
use crate::collision::{Hole, Wall};
use crate::physics::Vec2;

/// Canvas is 750 x 700. Walls are straight segments; holes are cups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoleLayout {
    pub id: u32,
    pub name: String,
    pub par: u32,
    pub start_pos: Vec2,
    pub walls: Vec<Wall>,
    pub holes: Vec<Hole>,
}

const CUP_RADIUS: f32 = 18.0;

/// Connected polyline through the points.
fn poly(pts: &[(f32, f32)]) -> Vec<Wall> {
    pts.windows(2)
        .map(|w| Wall { x1: w[0].0, y1: w[0].1, x2: w[1].0, y2: w[1].1 })
        .collect()
}

/// Closed rectangle.
fn rect(x: f32, y: f32, w: f32, h: f32) -> Vec<Wall> {
    poly(&[(x, y), (x + w, y), (x + w, y + h), (x, y + h), (x, y)])
}

/// Sine wall from x0 to x1 centred on y.
fn wave(x0: f32, x1: f32, y: f32, amp: f32, cycles: f32, steps: usize) -> Vec<Wall> {
    let pts: Vec<(f32, f32)> = (0..=steps)
        .map(|i| {
            let t = i as f32 / steps as f32;
            (x0 + t * (x1 - x0), y + (t * std::f32::consts::TAU * cycles).sin() * amp)
        })
        .collect();
    poly(&pts)
}

fn hole(id: u32, name: &str, par: u32, start: (f32, f32), cup: (f32, f32), walls: Vec<Vec<Wall>>) -> HoleLayout {
    HoleLayout {
        id,
        name: name.to_string(),
        par,
        start_pos: Vec2::new(start.0, start.1),
        walls: walls.concat(),
        holes: vec![Hole { x: cup.0, y: cup.1, radius: CUP_RADIUS, id: 1 }],
    }
}

/// The full 18-hole course, in play order.
pub fn course() -> Vec<HoleLayout> {
    vec![
        hole(1, "Starter Lane", 2, (110.0, 350.0), (620.0, 350.0), vec![
            rect(60.0, 250.0, 630.0, 200.0),
            rect(350.0, 320.0, 30.0, 60.0),
        ]),
        hole(2, "Two Steps", 3, (80.0, 620.0), (670.0, 80.0), vec![
            poly(&[(0.0, 470.0), (560.0, 470.0)]),
            poly(&[(190.0, 230.0), (750.0, 230.0)]),
        ]),
        hole(3, "Pillar Park", 3, (80.0, 350.0), (670.0, 350.0), vec![
            rect(330.0, 300.0, 60.0, 100.0),
            rect(480.0, 160.0, 40.0, 40.0),
            rect(480.0, 500.0, 40.0, 40.0),
        ]),
        hole(4, "Fence Bank", 3, (100.0, 120.0), (650.0, 560.0), vec![
            poly(&[(60.0, 330.0), (560.0, 330.0)]),
        ]),
        hole(5, "Snake", 5, (60.0, 640.0), (690.0, 60.0), vec![
            poly(&[(0.0, 560.0), (600.0, 560.0)]),
            poly(&[(150.0, 420.0), (750.0, 420.0)]),
            poly(&[(0.0, 280.0), (600.0, 280.0)]),
            poly(&[(150.0, 140.0), (750.0, 140.0)]),
        ]),
        hole(6, "The Wedge", 3, (120.0, 350.0), (630.0, 350.0), vec![
            poly(&[(300.0, 200.0), (380.0, 350.0), (300.0, 500.0)]),
        ]),
        hole(7, "Crossroads", 4, (120.0, 120.0), (640.0, 580.0), vec![
            poly(&[(375.0, 0.0), (375.0, 260.0)]),
            poly(&[(375.0, 440.0), (375.0, 700.0)]),
            poly(&[(0.0, 350.0), (260.0, 350.0)]),
            poly(&[(490.0, 350.0), (750.0, 350.0)]),
        ]),
        hole(8, "Icicle", 3, (80.0, 80.0), (660.0, 620.0), vec![
            poly(&[(250.0, 0.0), (250.0, 450.0)]),
            poly(&[(500.0, 250.0), (500.0, 700.0)]),
        ]),
        hole(9, "Blocks", 3, (100.0, 640.0), (650.0, 100.0), vec![
            rect(200.0, 400.0, 120.0, 120.0),
            rect(420.0, 240.0, 120.0, 120.0),
        ]),
        hole(10, "Castle Gate", 3, (100.0, 600.0), (375.0, 300.0), vec![
            poly(&[(200.0, 520.0), (200.0, 180.0), (550.0, 180.0), (550.0, 520.0), (380.0, 520.0)]),
        ]),
        hole(11, "Lanes", 5, (60.0, 60.0), (60.0, 600.0), vec![
            poly(&[(120.0, 130.0), (750.0, 130.0)]),
            poly(&[(0.0, 260.0), (630.0, 260.0)]),
            poly(&[(120.0, 390.0), (750.0, 390.0)]),
            poly(&[(0.0, 520.0), (630.0, 520.0)]),
        ]),
        hole(12, "Charon's Bridge", 3, (80.0, 350.0), (670.0, 350.0), vec![
            poly(&[(250.0, 200.0), (250.0, 500.0)]),
            poly(&[(500.0, 200.0), (500.0, 500.0)]),
        ]),
        hole(13, "Sine Serpent", 4, (40.0, 640.0), (710.0, 60.0), vec![
            wave(80.0, 680.0, 350.0, 80.0, 1.5, 60),
        ]),
        hole(14, "Stubs", 4, (80.0, 350.0), (690.0, 350.0), vec![
            poly(&[(220.0, 0.0), (220.0, 430.0)]),
            poly(&[(380.0, 270.0), (380.0, 700.0)]),
            poly(&[(540.0, 0.0), (540.0, 430.0)]),
        ]),
        hole(15, "Three Towers", 4, (80.0, 320.0), (680.0, 380.0), vec![
            rect(180.0, 130.0, 60.0, 60.0),
            rect(180.0, 510.0, 60.0, 60.0),
            rect(400.0, 300.0, 70.0, 100.0),
        ]),
        hole(16, "River Crossing", 4, (40.0, 360.0), (700.0, 600.0), vec![
            poly(&[(0.0, 300.0), (600.0, 300.0)]),
            poly(&[(150.0, 420.0), (750.0, 420.0)]),
        ]),
        hole(17, "The Hook", 3, (80.0, 80.0), (680.0, 620.0), vec![
            poly(&[(300.0, 0.0), (300.0, 500.0), (700.0, 500.0)]),
        ]),
        hole(18, "Finale", 5, (60.0, 650.0), (690.0, 60.0), vec![
            rect(240.0, 520.0, 40.0, 120.0),
            rect(420.0, 380.0, 40.0, 120.0),
            rect(330.0, 220.0, 40.0, 100.0),
            rect(560.0, 120.0, 40.0, 140.0),
        ]),
    ]
}
