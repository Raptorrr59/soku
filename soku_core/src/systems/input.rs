use hecs::World;
use crate::components::{Shape, Transform, Selectable, ZIndex};
use crate::systems::spatial::SpatialIndex;

fn point_in_triangle(px: f32, py: f32, x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
    let d1 = (px - x1) * (y0 - y1) - (x0 - x1) * (py - y1);
    let d2 = (px - x2) * (y1 - y2) - (x1 - x2) * (py - y2);
    let d3 = (px - x0) * (y2 - y0) - (x2 - x0) * (py - y0);

    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    !(has_neg && has_pos)
}

fn point_in_shape(px: f32, py: f32, transform: &Transform, shape: &Shape) -> bool {
    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();
    let dx = px - transform.x;
    let dy = py - transform.y;
    let lx = dx * cos + dy * sin;
    let ly = -dx * sin + dy * cos;

    match shape {
        Shape::Rectangle { width, height } => {
            lx >= -width / 2.0 && lx <= width / 2.0 &&
            ly >= -height / 2.0 && ly <= height / 2.0
        }
        Shape::Ellipse { radius_x, radius_y } => {
            if *radius_x <= 0.0 || *radius_y <= 0.0 { return false; }
            (lx * lx) / (radius_x * radius_x) + (ly * ly) / (radius_y * radius_y) <= 1.0
        }
        Shape::Triangle { base, height } => {
            point_in_triangle(lx, ly, 0.0, -height / 2.0, -base / 2.0, height / 2.0, base / 2.0, height / 2.0)
        }
        Shape::Polygon { radius_x, radius_y, .. } => {
            if *radius_x <= 0.0 || *radius_y <= 0.0 { return false; }
            (lx * lx) / (radius_x * radius_x) + (ly * ly) / (radius_y * radius_y) <= 1.0
        }
        Shape::Path { .. } => false,
    }
}

/// Updates the 'is_hovered' state of entities based on mouse coordinates using a spatial index.
pub fn update_hover_system_spatial(world: &mut World, spatial_index: &SpatialIndex, mouse_x: f32, mouse_y: f32) {
    // 1. Reset all hover states
    for (_id, selectable) in world.query_mut::<&mut Selectable>() {
        selectable.is_hovered = false;
    }

    // 2. Query spatial index for potential hits
    for spatial_obj in spatial_index.query_point(mouse_x, mouse_y) {
        if let Ok(mut selectable) = world.get::<&mut Selectable>(spatial_obj.entity) {
            // Precise hit detection (AABB is just the first pass)
            if let (Ok(transform), Ok(shape)) = (world.get::<&Transform>(spatial_obj.entity), world.get::<&Shape>(spatial_obj.entity)) {
                if point_in_shape(mouse_x, mouse_y, &transform, &shape) {
                    selectable.is_hovered = true;
                }
            }
        }
    }
}

/// Updates the 'is_selected' state using a spatial index.
pub fn update_selection_system_spatial(world: &mut World, spatial_index: &SpatialIndex, mouse_x: f32, mouse_y: f32) {
    // 1. Update hover first to get the latest candidates
    update_hover_system_spatial(world, spatial_index, mouse_x, mouse_y);

    // 2. Reset all selections
    for (_id, selectable) in world.query_mut::<&mut Selectable>() {
        selectable.is_selected = false;
    }

    // 3. Find the hovered one with highest ZIndex
    let top_hovered = {
        world.query_mut::<(&Selectable, &ZIndex)>()
            .into_iter()
            .filter(|(_, (s, _))| s.is_hovered)
            .max_by(|(_, (_, z1)), (_, (_, z2))| z1.0.partial_cmp(&z2.0).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(e, _)| e)
    };

    // 4. Select it
    if let Some(entity) = top_hovered {
        if let Ok(mut selectable) = world.get::<&mut Selectable>(entity) {
            selectable.is_selected = true;
        }
    }
}

/// Updates the 'is_hovered' state of entities based on mouse coordinates.
pub fn update_hover_system(world: &mut World, mouse_x: f32, mouse_y: f32) {
    for (_id, (transform, shape, selectable)) in world.query_mut::<(&Transform, &Shape, &mut Selectable)>() {
        selectable.is_hovered = point_in_shape(mouse_x, mouse_y, transform, shape);
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
