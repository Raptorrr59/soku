use hecs::World;
use crate::components::{Shape, Transform, Selectable};

/// Iterates over all entities with a Transform and a Shape,
/// packing their data into a flat f32 buffer for the Wasm bridge.
pub fn pack_render_buffer(world: &World, buffer: &mut Vec<f32>) {
    buffer.clear();
    
    for (entity, (transform, shape, selectable)) in world.query::<(&Transform, &Shape, &Selectable)>().iter() {
        let id_f32 = entity.id() as f32;
        
        // Pack state into the type float using decimals for flags:
        // Type.0 = Rectangle (1.0), Circle (2.0)
        // + 0.1 if hovered
        // + 0.2 if selected
        let mut type_val = match shape {
            Shape::Rectangle { .. } => 1.0,
            Shape::Circle { .. } => 2.0,
            _ => 0.0,
        };
        
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
            _ => {}
        }
    }
}
