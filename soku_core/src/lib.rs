pub mod commands;
pub mod components;
pub mod systems;

use hecs::{World, Entity};
use crate::components::{Shape, Transform, Selectable, Camera};

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
        
        // Initial shapes
        world.spawn((
            Transform { x: 100.0, y: 150.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            Shape::Rectangle { width: 200.0, height: 100.0 },
            Selectable { is_selected: false, is_hovered: false },
        ));
        
        world.spawn((
            Transform { x: 400.0, y: 300.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            Shape::Circle { radius: 80.0 },
            Selectable { is_selected: false, is_hovered: false },
        ));

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
            
            // Scoped find to drop borrow
            let target = {
                self.world.query_mut::<&Selectable>()
                    .into_iter()
                    .find(|(_, s)| s.is_hovered)
                    .map(|(e, _)| e)
            };
            self.drag_target = target;
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
            _ => Shape::Rectangle { width: 50.0, height: 50.0 },
        };

        self.world.spawn((
            Transform { x, y, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            shape,
            Selectable { is_selected: false, is_hovered: false },
        ));
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

    pub fn render(&self, buffer: &mut Vec<f32>) {
        systems::render::pack_render_buffer(&self.world, buffer);
    }

    pub fn get_camera(&self) -> Camera {
        *self.world.get::<&Camera>(self.camera).unwrap()
    }
}
