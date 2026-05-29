use hecs::World;
use crate::components::{Shape, Transform, Selectable, ZIndex, Renderable};
use crate::systems::spatial::{SpatialIndex, get_entity_handles, HandleType};

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
    spatial_index: &SpatialIndex,
) {
    buffer.clear();
    
    // 1. Query spatial index for visible entities
    let mut visible_entities: Vec<_> = Vec::new();
    for spatial_obj in spatial_index.query_aabb(min_x, min_y, max_x, max_y) {
        if let (Ok(transform), Ok(shape), Ok(selectable), Ok(zindex), Ok(renderable)) = (
            world.get::<&Transform>(spatial_obj.entity),
            world.get::<&Shape>(spatial_obj.entity),
            world.get::<&Selectable>(spatial_obj.entity),
            world.get::<&ZIndex>(spatial_obj.entity),
            world.get::<&Renderable>(spatial_obj.entity),
        ) {
            visible_entities.push((spatial_obj.entity, (transform, shape, selectable, zindex, renderable)));
        }
    }

    // 2. Sort by ZIndex (low to high for correct painter's algorithm)
    visible_entities.sort_by(|(_, (_, _, _, z1, _)), (_, (_, _, _, z2, _))| {
        z1.0.partial_cmp(&z2.0).unwrap_or(std::cmp::Ordering::Equal)
    });

    // 3. Pack into buffer
    for (entity, (transform, shape, selectable, _, renderable)) in visible_entities {
        let id_f32 = entity.id() as f32;
        
        let colors = vec!["#3b82f6", "#ec4899", "#10b981", "#f59e0b", "#8b5cf6"];
        let color_idx = colors.iter().position(|&c| c == renderable.color).unwrap_or(0) as f32;

        let type_val = match &*shape {
            Shape::Rectangle { .. } => 1.0,
            Shape::Ellipse { .. } => 2.0,
            Shape::Triangle { .. } => 3.0,
            Shape::Polygon { sides, .. } => 4.0 + (*sides as f32 * 0.01),
            _ => 0.0,
        };

        let mut flags = 0.0;
        if selectable.is_hovered { flags += 1.0; }
        if selectable.is_selected { flags += 2.0; }

        match &*shape {
            Shape::Rectangle { width, height } => {
                buffer.push(id_f32);
                buffer.push(type_val);
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*width);
                buffer.push(*height);
                buffer.push(color_idx);
                buffer.push(flags);
                buffer.push(transform.rotation);
            }
            Shape::Ellipse { radius_x, radius_y } => {
                buffer.push(id_f32);
                buffer.push(type_val);
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*radius_x);
                buffer.push(*radius_y);
                buffer.push(color_idx);
                buffer.push(flags);
                buffer.push(transform.rotation);
            }
            Shape::Triangle { base, height } => {
                buffer.push(id_f32);
                buffer.push(type_val);
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*base);
                buffer.push(*height);
                buffer.push(color_idx);
                buffer.push(flags);
                buffer.push(transform.rotation);
            }
            Shape::Polygon { radius_x, radius_y, .. } => {
                buffer.push(id_f32);
                buffer.push(type_val);
                buffer.push(transform.x);
                buffer.push(transform.y);
                buffer.push(*radius_x);
                buffer.push(*radius_y);
                buffer.push(color_idx);
                buffer.push(flags);
                buffer.push(transform.rotation);
            }
            _ => {}
        }
    }

    // 4. Pack Handles for selected entities
    for (entity, (transform, shape, selectable)) in world.query::<(&Transform, &Shape, &Selectable)>().iter() {
        if !selectable.is_selected { continue; }
        
        let handles = get_entity_handles(transform, shape);

        for (h_type, h_x, h_y) in handles {
            let sub_type = match h_type {
                HandleType::TopLeft => 1.0,
                HandleType::TopRight => 2.0,
                HandleType::BottomLeft => 3.0,
                HandleType::BottomRight => 4.0,
                HandleType::Rotation => 5.0,
            };

            buffer.push(entity.id() as f32);
            buffer.push(5.0); // Type 5 = Handle
            buffer.push(h_x);
            buffer.push(h_y);
            buffer.push(10.0); // handle size width
            buffer.push(10.0); // handle size height
            buffer.push(sub_type); // handle sub-type
            buffer.push(0.0); // flags
            buffer.push(0.0); // rotation placeholder
        }
    }
}
