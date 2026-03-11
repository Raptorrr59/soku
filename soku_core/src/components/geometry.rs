use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
    Path { points: Vec<(f32, f32)> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Renderable {
    pub color: String,
    pub stroke_width: f32,
    pub fill: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Selectable {
    pub is_selected: bool,
    pub is_hovered: bool,
}
