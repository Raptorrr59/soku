pub mod commands;
pub mod components;
pub mod systems;

use hecs::{World, Entity};
use crate::components::{Shape, Transform, Selectable, Camera, ZIndex, Renderable, SpatialMetadata};
use crate::systems::spatial::{SpatialIndex, calculate_aabb, get_entity_handles, HandleType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DragMode {
    None,
    Translate,
    Resize(HandleType),
}

/// The central state of the Soku engine
pub struct SokuEngine {
    world: World,
    mouse_x: f32,
    mouse_y: f32,
    drag_target: Option<Entity>,
    drag_mode: DragMode,
    camera: Entity,
    spatial_index: SpatialIndex,
}

impl SokuEngine {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Create camera entity
        let camera = world.spawn((
            Camera { x: 0.0, y: 0.0, zoom: 1.0 },
        ));

        let mut spatial_index = SpatialIndex::new();
        spatial_index.build_from_world(&world);

        Self {
            world,
            mouse_x: 0.0,
            mouse_y: 0.0,
            drag_target: None,
            drag_mode: DragMode::None,
            camera,
            spatial_index,
        }
    }

    pub fn step(&mut self, mouse_x: f32, mouse_y: f32, mouse_down: bool, mouse_up: bool) {
        let camera = self.get_camera();
        
        // Transform screen space to world space for the engine's internal logic
        let world_mouse_x = (mouse_x / camera.zoom) + camera.x;
        let world_mouse_y = (mouse_y / camera.zoom) + camera.y;

        // Current world-space movement delta
        let dx = (mouse_x - self.mouse_x) / camera.zoom;
        let dy = (mouse_y - self.mouse_y) / camera.zoom;
        
        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;

        // 1. Handle Mouse Up (release drag)
        if mouse_up {
            self.drag_target = None;
            self.drag_mode = DragMode::None;
        }

        // 2. Handle Mouse Down (start drag & selection)
        if mouse_down {
            // Check handles first!
            let handle_hit = self.find_handle_at(world_mouse_x, world_mouse_y);
            
            if let Some((entity, handle)) = handle_hit {
                self.drag_target = Some(entity);
                self.drag_mode = DragMode::Resize(handle);
            } else {
                systems::input::update_selection_system_spatial(&mut self.world, &self.spatial_index, world_mouse_x, world_mouse_y);
                
                // Find the hovered entity with the HIGHEST ZIndex
                let target = {
                    self.world.query_mut::<(&Selectable, &ZIndex)>()
                        .into_iter()
                        .filter(|(_, (s, _))| s.is_hovered)
                        .max_by(|(_, (_, z1)), (_, (_, z2))| z1.0.partial_cmp(&z2.0).unwrap_or(std::cmp::Ordering::Equal))
                        .map(|(e, _)| e)
                };
                
                if let Some(e) = target {
                    self.drag_target = Some(e);
                    self.drag_mode = DragMode::Translate;
                }
            }
        }

        // 3. Handle Dragging or Hovering
        if let Some(entity) = self.drag_target {
            match self.drag_mode {
                DragMode::Translate => {
                    if let Ok(mut transform) = self.world.get::<&mut Transform>(entity) {
                        transform.x += dx;
                        transform.y += dy;
                    }
                }
                DragMode::Resize(handle) => {
                    if let (Ok(mut transform), Ok(mut shape)) = (self.world.get::<&mut Transform>(entity), self.world.get::<&mut Shape>(entity)) {
                        match handle {
                            HandleType::Rotation => {
                                let target_x = world_mouse_x - transform.x;
                                let target_y = world_mouse_y - transform.y;
                                transform.rotation = target_y.atan2(target_x) + std::f32::consts::FRAC_PI_2;
                            }
                            _ => {
                                let cos = transform.rotation.cos();
                                let sin = transform.rotation.sin();
                                let local_dx = dx * cos + dy * sin;
                                let local_dy = -dx * sin + dy * cos;

                                match &mut *shape {
                                    Shape::Rectangle { width, height } => {
                                        let old_w = *width;
                                        let old_h = *height;
                                        let (dw, dh) = match handle {
                                            HandleType::TopLeft => (-local_dx, -local_dy),
                                            HandleType::TopRight => (local_dx, -local_dy),
                                            HandleType::BottomLeft => (-local_dx, local_dy),
                                            HandleType::BottomRight => (local_dx, local_dy),
                                            _ => (0.0, 0.0),
                                        };
                                        let mut new_w = old_w + dw;
                                        let mut new_h = old_h + dh;
                                        if new_w < 5.0 { new_w = 5.0; }
                                        if new_h < 5.0 { new_h = 5.0; }
                                        let actual_dw = new_w - old_w;
                                        let actual_dh = new_h - old_h;

                                        let (local_cx, local_cy) = match handle {
                                            HandleType::TopLeft => (-actual_dw / 2.0, -actual_dh / 2.0),
                                            HandleType::TopRight => (actual_dw / 2.0, -actual_dh / 2.0),
                                            HandleType::BottomLeft => (-actual_dw / 2.0, actual_dh / 2.0),
                                            HandleType::BottomRight => (actual_dw / 2.0, actual_dh / 2.0),
                                            _ => (0.0, 0.0),
                                        };

                                        *width = new_w;
                                        *height = new_h;

                                        transform.x += local_cx * cos - local_cy * sin;
                                        transform.y += local_cx * sin + local_cy * cos;
                                    }
                                    Shape::Ellipse { radius_x, radius_y } => {
                                        let old_w = *radius_x * 2.0;
                                        let old_h = *radius_y * 2.0;
                                        let (dw, dh) = match handle {
                                            HandleType::TopLeft => (-local_dx, -local_dy),
                                            HandleType::TopRight => (local_dx, -local_dy),
                                            HandleType::BottomLeft => (-local_dx, local_dy),
                                            HandleType::BottomRight => (local_dx, local_dy),
                                            _ => (0.0, 0.0),
                                        };
                                        let mut new_w = old_w + dw;
                                        let mut new_h = old_h + dh;
                                        if new_w < 4.0 { new_w = 4.0; }
                                        if new_h < 4.0 { new_h = 4.0; }
                                        let actual_dw = new_w - old_w;
                                        let actual_dh = new_h - old_h;

                                        let (local_cx, local_cy) = match handle {
                                            HandleType::TopLeft => (-actual_dw / 2.0, -actual_dh / 2.0),
                                            HandleType::TopRight => (actual_dw / 2.0, -actual_dh / 2.0),
                                            HandleType::BottomLeft => (-actual_dw / 2.0, actual_dh / 2.0),
                                            HandleType::BottomRight => (actual_dw / 2.0, actual_dh / 2.0),
                                            _ => (0.0, 0.0),
                                        };

                                        *radius_x = new_w / 2.0;
                                        *radius_y = new_h / 2.0;

                                        transform.x += local_cx * cos - local_cy * sin;
                                        transform.y += local_cx * sin + local_cy * cos;
                                    }
                                    Shape::Triangle { base, height } => {
                                        let old_w = *base;
                                        let old_h = *height;
                                        let (dw, dh) = match handle {
                                            HandleType::TopLeft => (-local_dx, -local_dy),
                                            HandleType::TopRight => (local_dx, -local_dy),
                                            HandleType::BottomLeft => (-local_dx, local_dy),
                                            HandleType::BottomRight => (local_dx, local_dy),
                                            _ => (0.0, 0.0),
                                        };
                                        let mut new_w = old_w + dw;
                                        let mut new_h = old_h + dh;
                                        if new_w < 5.0 { new_w = 5.0; }
                                        if new_h < 5.0 { new_h = 5.0; }
                                        let actual_dw = new_w - old_w;
                                        let actual_dh = new_h - old_h;

                                        let (local_cx, local_cy) = match handle {
                                            HandleType::TopLeft => (-actual_dw / 2.0, -actual_dh / 2.0),
                                            HandleType::TopRight => (actual_dw / 2.0, -actual_dh / 2.0),
                                            HandleType::BottomLeft => (-actual_dw / 2.0, actual_dh / 2.0),
                                            HandleType::BottomRight => (actual_dw / 2.0, actual_dh / 2.0),
                                            _ => (0.0, 0.0),
                                        };

                                        *base = new_w;
                                        *height = new_h;

                                        transform.x += local_cx * cos - local_cy * sin;
                                        transform.y += local_cx * sin + local_cy * cos;
                                    }
                                    Shape::Polygon { radius_x, radius_y, .. } => {
                                        let old_w = *radius_x * 2.0;
                                        let old_h = *radius_y * 2.0;
                                        let (dw, dh) = match handle {
                                            HandleType::TopLeft => (-local_dx, -local_dy),
                                            HandleType::TopRight => (local_dx, -local_dy),
                                            HandleType::BottomLeft => (-local_dx, local_dy),
                                            HandleType::BottomRight => (local_dx, local_dy),
                                            _ => (0.0, 0.0),
                                        };
                                        let mut new_w = old_w + dw;
                                        let mut new_h = old_h + dh;
                                        if new_w < 4.0 { new_w = 4.0; }
                                        if new_h < 4.0 { new_h = 4.0; }
                                        let actual_dw = new_w - old_w;
                                        let actual_dh = new_h - old_h;

                                        let (local_cx, local_cy) = match handle {
                                            HandleType::TopLeft => (-actual_dw / 2.0, -actual_dh / 2.0),
                                            HandleType::TopRight => (actual_dw / 2.0, -actual_dh / 2.0),
                                            HandleType::BottomLeft => (-actual_dw / 2.0, actual_dh / 2.0),
                                            HandleType::BottomRight => (actual_dw / 2.0, actual_dh / 2.0),
                                            _ => (0.0, 0.0),
                                        };

                                        *radius_x = new_w / 2.0;
                                        *radius_y = new_h / 2.0;

                                        transform.x += local_cx * cos - local_cy * sin;
                                        transform.y += local_cx * sin + local_cy * cos;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                _ => {}
            }

            // Surgical R-Tree update: Remove old, move, insert new
            let mut old_meta = None;
            if let Ok(meta) = self.world.get::<&SpatialMetadata>(entity) {
                old_meta = Some(*meta);
            }

            if let Some(meta) = old_meta {
                self.spatial_index.remove(entity, meta.min, meta.max);
            }

            if let (Ok(transform), Ok(shape)) = (self.world.get::<&Transform>(entity), self.world.get::<&Shape>(entity)) {
                let aabb = calculate_aabb(&transform, &shape);
                let new_min = aabb.lower();
                let new_max = aabb.upper();
                
                self.spatial_index.insert(entity, &transform, &shape);
                
                // Drop borrows before updating meta
                drop(transform);
                drop(shape);
                if let Ok(mut meta) = self.world.get::<&mut SpatialMetadata>(entity) {
                    meta.min = new_min;
                    meta.max = new_max;
                }
            }
        } else {
            systems::input::update_hover_system_spatial(&mut self.world, &self.spatial_index, world_mouse_x, world_mouse_y);
        }
    }

    fn find_handle_at(&self, x: f32, y: f32) -> Option<(Entity, HandleType)> {
        let handle_size = 10.0;
        
        // Only check selected entities
        for (entity, (transform, shape, selectable)) in self.world.query::<(&Transform, &Shape, &Selectable)>().iter() {
            if !selectable.is_selected { continue; }
            
            let handles = get_entity_handles(transform, shape);
            for (h_type, h_x, h_y) in handles {
                if x >= h_x - handle_size/2.0 && x <= h_x + handle_size/2.0 &&
                   y >= h_y - handle_size/2.0 && y <= h_y + handle_size/2.0 {
                    return Some((entity, h_type));
                }
            }
        }
        None
    }

    pub fn spawn_shape(&mut self, shape_type: u8, x: f32, y: f32) {
        let shape = match shape_type {
            1 => Shape::Rectangle { width: 100.0, height: 100.0 },
            2 => Shape::Ellipse { radius_x: 50.0, radius_y: 50.0 },
            3 => Shape::Triangle { base: 100.0, height: 100.0 },
            4 => Shape::Polygon { sides: 6, radius_x: 50.0, radius_y: 50.0 },
            _ => Shape::Rectangle { width: 50.0, height: 50.0 },
        };

        // For new shapes, pick a ZIndex higher than existing ones
        let max_z = self.world.query::<&ZIndex>()
            .iter()
            .map(|(_, z)| z.0)
            .fold(0.0, f32::max);

        let aabb = calculate_aabb(&Transform { x, y, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 }, &shape);
        let meta = SpatialMetadata { min: aabb.lower(), max: aabb.upper() };

        let entity = self.world.spawn((
            Transform { x, y, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            shape.clone(),
            Selectable { is_selected: false, is_hovered: false },
            ZIndex(max_z + 1.0),
            Renderable { 
                color: "#3b82f6".to_string(), 
                stroke_width: 1.5,
                fill: true 
            },
            meta,
        ));

        if let Ok(transform) = self.world.get::<&Transform>(entity) {
            self.spatial_index.insert(entity, &transform, &shape);
        }
    }

    pub fn update_selected_color(&mut self, color_hex: &str) {
        let selected: Vec<Entity> = self.world.query::<&Selectable>()
            .iter()
            .filter(|(_, s)| s.is_selected)
            .map(|(e, _)| e)
            .collect();
        
        for entity in selected {
            if let Ok(mut renderable) = self.world.get::<&mut Renderable>(entity) {
                renderable.color = color_hex.to_string();
            }
        }
    }

    pub fn update_selected_zindex(&mut self, delta: f32) {
        let selected: Vec<Entity> = self.world.query::<&Selectable>()
            .iter()
            .filter(|(_, s)| s.is_selected)
            .map(|(e, _)| e)
            .collect();
        
        for entity in selected {
            if let Ok(mut z) = self.world.get::<&mut ZIndex>(entity) {
                z.0 += delta;
            }
        }
    }

    pub fn resize_selected(&mut self, factor: f32) {
        let selected: Vec<Entity> = self.world.query::<&Selectable>()
            .iter()
            .filter(|(_, s)| s.is_selected)
            .map(|(e, _)| e)
            .collect();
        
        for entity in selected {
            if let Ok(mut shape) = self.world.get::<&mut Shape>(entity) {
                match *shape {
                    Shape::Rectangle { ref mut width, ref mut height } => {
                        *width *= factor;
                        *height *= factor;
                    }
                    Shape::Ellipse { ref mut radius_x, ref mut radius_y } => {
                        *radius_x *= factor;
                        *radius_y *= factor;
                    }
                    Shape::Triangle { ref mut base, ref mut height } => {
                        *base *= factor;
                        *height *= factor;
                    }
                    Shape::Polygon { ref mut radius_x, ref mut radius_y, .. } => {
                        *radius_x *= factor;
                        *radius_y *= factor;
                    }
                    _ => {}
                }
            }
        }
        self.spatial_index.build_from_world(&self.world);
    }

    pub fn delete_selected(&mut self) {
        let to_delete: Vec<Entity> = self.world.query::<&Selectable>()
            .iter()
            .filter(|(_, s)| s.is_selected)
            .map(|(e, _)| e)
            .collect();
        
        for entity in to_delete {
            let _ = self.world.despawn(entity);
        }
        self.drag_target = None;
        self.spatial_index.build_from_world(&self.world);
    }

    pub fn move_camera(&mut self, dx: f32, dy: f32) {
        if let Ok(mut camera) = self.world.get::<&mut Camera>(self.camera) {
            camera.x += dx;
            camera.y += dy;
        }
    }

    pub fn zoom_camera(&mut self, delta: f32, mouse_x: f32, mouse_y: f32) {
        if let Ok(mut camera) = self.world.get::<&mut Camera>(self.camera) {
            let zoom_old = camera.zoom;
            let zoom_new = zoom_old * delta;
            camera.x += (mouse_x / zoom_old) - (mouse_x / zoom_new);
            camera.y += (mouse_y / zoom_old) - (mouse_y / zoom_new);
            camera.zoom = zoom_new;
        }
    }

    pub fn render(&self, buffer: &mut Vec<f32>, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        systems::render::pack_render_buffer(&self.world, buffer, min_x, min_y, max_x, max_y, &self.spatial_index);
    }

    pub fn get_camera(&self) -> Camera {
        *self.world.get::<&Camera>(self.camera).unwrap()
    }
}
