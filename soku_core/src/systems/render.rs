use hecs::World;
use crate::components::{Shape, Transform};

/// Iterates over all entities with a Transform and a Shape,
/// packing their data into a flat f32 buffer for the Wasm bridge.
pub fn pack_render_buffer(world: &World, buffer: &mut Vec<f32>) {
    buffer.clear();
    
    for (entity, (transform, shape)) in world.query::<(&Transform, &Shape)>().iter() {
        // hecs::Entity is composed of an ID and a Generation. 
        // We use the ID part as a stable index for rendering.
        let id_f32 = entity.id() as f32;
        
        match shape {
            Shape::Rectangle { width, height } => {
                buffer.push(id_f32);
                buffer.push(1.0); // Type 1 = Rectangle
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*width);
                buffer.push(*height);
            }
            Shape::Circle { radius } => {
                buffer.push(id_f32);
                buffer.push(2.0); // Type 2 = Circle
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*radius);
                buffer.push(0.0); // Padding to maintain stride of 6
            }
            Shape::Path { .. } => {
                // Not implemented in the flat buffer yet
            }
        }
    }
}
