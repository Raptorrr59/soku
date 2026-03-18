pub mod commands;
pub mod components;
pub mod systems;

use hecs::{World, Entity};
use crate::components::{Shape, Transform, Selectable, Camera, ZIndex, Renderable};

/// The central state of the Soku engine
pub struct SokuEngine {
    world: World,
    mouse_x: f32,
    mouse_y: f32,
    drag_target: Option<Entity>,
    camera: Entity,
}

impl SokuEngine {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Create camera entity
        let camera = world.spawn((
            Camera { x: 0.0, y: 0.0, zoom: 1.0 },
        ));

        Self {
            world,
            mouse_x: 0.0,
            mouse_y: 0.0,
            drag_target: None,
            camera,
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
        }

        // 2. Handle Mouse Down (start drag & selection)
        if mouse_down {
            systems::input::update_selection_system(&mut self.world);
            
            // Find the hovered entity with the HIGHEST ZIndex
            let target = {
                self.world.query_mut::<(&Selectable, &ZIndex)>()
                    .into_iter()
                    .filter(|(_, (s, _))| s.is_hovered)
                    .max_by(|(_, (_, z1)), (_, (_, z2))| z1.0.partial_cmp(&z2.0).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(e, _)| e)
            };
            
            if let Some(e) = target {
                // When we start dragging, we should also bring it to top? 
                // Let's just select it for now.
                self.drag_target = Some(e);
            }
        }

        // 3. Handle Dragging or Hovering
        if let Some(entity) = self.drag_target {
            if let Ok(mut transform) = self.world.get::<&mut Transform>(entity) {
                transform.x += dx;
                transform.y += dy;
            }
        } else {
            systems::input::update_hover_system(&mut self.world, world_mouse_x, world_mouse_y);
        }
    }

    pub fn spawn_shape(&mut self, shape_type: u8, x: f32, y: f32) {
        let shape = match shape_type {
            1 => Shape::Rectangle { width: 100.0, height: 100.0 },
            2 => Shape::Circle { radius: 50.0 },
            3 => Shape::Triangle { base: 100.0, height: 100.0 },
            4 => Shape::Polygon { sides: 6, radius: 50.0 },
            _ => Shape::Rectangle { width: 50.0, height: 50.0 },
        };

        // For new shapes, pick a ZIndex higher than existing ones
        let max_z = self.world.query::<&ZIndex>()
            .iter()
            .map(|(_, z)| z.0)
            .fold(0.0, f32::max);

        self.world.spawn((
            Transform { x, y, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            shape,
            Selectable { is_selected: false, is_hovered: false },
            ZIndex(max_z + 1.0),
            Renderable { 
                color: "#3b82f6".to_string(), 
                stroke_width: 1.5,
                fill: true 
            },
        ));
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
                    Shape::Circle { ref mut radius } => {
                        *radius *= factor;
                    }
                    Shape::Triangle { ref mut base, ref mut height } => {
                        *base *= factor;
                        *height *= factor;
                    }
                    Shape::Polygon { ref mut radius, .. } => {
                        *radius *= factor;
                    }
                    _ => {}
                }
            }
        }
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
    }

    pub fn move_camera(&mut self, dx: f32, dy: f32) {
        if let Ok(mut camera) = self.world.get::<&mut Camera>(self.camera) {
            camera.x += dx;
            camera.y += dy;
        }
    }

    pub fn zoom_camera(&mut self, delta: f32) {
        if let Ok(mut camera) = self.world.get::<&mut Camera>(self.camera) {
            camera.zoom *= delta;
        }
    }

    pub fn render(&self, buffer: &mut Vec<f32>, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        systems::render::pack_render_buffer(&self.world, buffer, min_x, min_y, max_x, max_y);
    }

    pub fn get_camera(&self) -> Camera {
        *self.world.get::<&Camera>(self.camera).unwrap()
    }
}
