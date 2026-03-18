use hecs::World;
use crate::components::{Shape, Transform, Selectable, ZIndex, Renderable};

/// Iterates over all entities with a Transform and a Shape,
/// packing their data into a flat f32 buffer for the Wasm bridge.
/// Only entities within the provided viewport bounds are included.
pub fn pack_render_buffer(
    world: &World, 
    buffer: &mut Vec<f32>,
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
) {
    buffer.clear();
    
    // 1. Collect and cull entities
    let mut query = world.query::<(&Transform, &Shape, &Selectable, &ZIndex, &Renderable)>();
    let mut visible_entities: Vec<_> = query
        .into_iter()
        .filter(|(_, (transform, shape, _, _, _))| {
            let (width, height) = match shape {
                Shape::Rectangle { width, height } => (*width, *height),
                Shape::Circle { radius } => (*radius * 2.0, *radius * 2.0),
                Shape::Triangle { base, height } => (*base, *height),
                Shape::Polygon { radius, .. } => (*radius * 2.0, *radius * 2.0),
                _ => (0.0, 0.0),
            };
            
            !(transform.x + width < min_x || transform.x > max_x ||
              transform.y + height < min_y || transform.y > max_y)
        })
        .collect();

    // 2. Sort by ZIndex (low to high for correct painter's algorithm)
    visible_entities.sort_by(|(_, (_, _, _, z1, _)), (_, (_, _, _, z2, _))| {
        z1.0.partial_cmp(&z2.0).unwrap_or(std::cmp::Ordering::Equal)
    });

    // 3. Pack into buffer
    for (entity, (transform, shape, selectable, _, renderable)) in visible_entities {
        let id_f32 = entity.id() as f32;
        
        let colors = vec!["#3b82f6", "#ec4899", "#10b981", "#f59e0b", "#8b5cf6"];
        let color_idx = colors.iter().position(|&c| c == renderable.color).unwrap_or(0) as f32;

        // Pack state into the type float:
        // Type.0 = Rectangle (1.0), Circle (2.0), ...
        // Type.XX = color_idx * 0.01
        // + 0.1 if hovered
        // + 0.2 if selected
        let mut type_val = match shape {
            Shape::Rectangle { .. } => 1.0,
            Shape::Circle { .. } => 2.0,
            Shape::Triangle { .. } => 3.0,
            Shape::Polygon { .. } => 4.0,
            _ => 0.0,
        };
        
        type_val += color_idx * 0.01;
        if selectable.is_hovered { type_val += 0.1; }
        if selectable.is_selected { type_val += 0.2; }
        
        match shape {
            Shape::Rectangle { width, height } => {
                buffer.push(id_f32);
                buffer.push(type_val);
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*width);
                buffer.push(*height);
            }
            Shape::Circle { radius } => {
                buffer.push(id_f32);
                buffer.push(type_val);
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*radius);
                buffer.push(0.0);
            }
            Shape::Triangle { base, height } => {
                buffer.push(id_f32);
                buffer.push(type_val);
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*base);
                buffer.push(*height);
            }
            Shape::Polygon { sides, radius } => {
                buffer.push(id_f32);
                buffer.push(type_val);
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*radius);
                buffer.push(*sides as f32);
            }
            _ => {}
        }
    }
}
