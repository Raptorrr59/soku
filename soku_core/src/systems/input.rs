use hecs::World;
use crate::components::{Shape, Transform, Selectable, ZIndex};

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
            Shape::Triangle { base, height } => {
                // Simplified AABB check for Triangle for now
                mouse_x >= transform.x - base / 2.0 && mouse_x <= transform.x + base / 2.0 &&
                mouse_y >= transform.y - height / 2.0 && mouse_y <= transform.y + height / 2.0
            }
            Shape::Polygon { radius, .. } => {
                let dx = mouse_x - transform.x;
                let dy = mouse_y - transform.y;
                (dx * dx + dy * dy).sqrt() <= *radius
            }
            Shape::Path { .. } => false,
        };
        
        selectable.is_hovered = is_inside;
    }
}

/// Updates the 'is_selected' state. Finds the ONE hovered entity with highest ZIndex and selects it.
pub fn update_selection_system(world: &mut World) {
    // 1. Reset all selections
    for (_id, selectable) in world.query_mut::<&mut Selectable>() {
        selectable.is_selected = false;
    }

    // 2. Find the hovered one with highest ZIndex
    let top_hovered = {
        world.query_mut::<(&Selectable, &ZIndex)>()
            .into_iter()
            .filter(|(_, (s, _))| s.is_hovered)
            .max_by(|(_, (_, z1)), (_, (_, z2))| z1.0.partial_cmp(&z2.0).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(e, _)| e)
    };

    // 3. Select it
    if let Some(entity) = top_hovered {
        if let Ok(mut selectable) = world.get::<&mut Selectable>(entity) {
            selectable.is_selected = true;
        }
    }
}
