use hecs::World;
use crate::components::{Shape, Transform, Selectable};

/// Updates the 'is_hovered' state of entities based on mouse coordinates.
pub fn update_hover_system(world: &mut World, mouse_x: f32, mouse_y: f32) {
    for (_id, (transform, shape, selectable)) in world.query_mut::<(&Transform, &Shape, &mut Selectable)>() {
        let is_inside = match shape {
            Shape::Rectangle { width, height } => {
                mouse_x >= transform.x && mouse_x <= transform.x + width &&
                mouse_y >= transform.y && mouse_y <= transform.y + height
            }
            Shape::Circle { radius } => {
                let dx = mouse_x - transform.x;
                let dy = mouse_y - transform.y;
                (dx * dx + dy * dy).sqrt() <= *radius
            }
            Shape::Path { .. } => false, // TODO: Path hit detection is complex
        };
        
        selectable.is_hovered = is_inside;
    }
}

/// Updates the 'is_selected' state. For now, just selects whatever is hovered.
pub fn update_selection_system(world: &mut World) {
    for (_id, selectable) in world.query_mut::<&mut Selectable>() {
        selectable.is_selected = selectable.is_hovered;
    }
}
