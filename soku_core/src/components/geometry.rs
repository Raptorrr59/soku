use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
    Triangle { base: f32, height: f32 },
    Polygon { sides: u32, radius: f32 },
    Path { points: Vec<(f32, f32)> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Renderable {
    pub color: String,
    pub stroke_width: f32,
    pub fill: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ZIndex(pub f32);
