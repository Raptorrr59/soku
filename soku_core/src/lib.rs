pub mod commands;
pub mod components;
pub mod systems;

use hecs::{World, Entity};
use commands::HistoryManager;
use components::{Shape, Transform, Selectable};

/// The central state of the Soku engine
pub struct SokuEngine {
    pub world: World,
    pub history: HistoryManager,
    
    // Internal Input State
    mouse_x: f32,
    mouse_y: f32,
    next_mouse_pos: Option<(f32, f32)>,
    mouse_down_triggered: bool,
    mouse_up_triggered: bool,
    
    pub drag_target: Option<Entity>,
}

impl SokuEngine {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Spawn some initial shapes
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
        
        world.spawn((
            Transform { x: 600.0, y: 100.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            Shape::Rectangle { width: 120.0, height: 120.0 },
            Selectable { is_selected: false, is_hovered: false },
        ));

        world.spawn((
            Transform { x: 200.0, y: 400.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            Shape::Circle { radius: 50.0 },
            Selectable { is_selected: false, is_hovered: false },
        ));

        Self {
            world,
            history: HistoryManager::new(),
            mouse_x: 0.0,
            mouse_y: 0.0,
            next_mouse_pos: None,
            mouse_down_triggered: false,
            mouse_up_triggered: false,
            drag_target: None,
        }
    }

    /// The single source of truth for engine updates. 
    /// Processes all queued inputs and updates the ECS world.
    pub fn step(&mut self) {
        // 1. Process Mouse Move
        if let Some((nx, ny)) = self.next_mouse_pos.take() {
            let dx = nx - self.mouse_x;
            let dy = ny - self.mouse_y;
            self.mouse_x = nx;
            self.mouse_y = ny;

            if let Some(entity) = self.drag_target {
                if let Ok(mut transform) = self.world.get::<&mut Transform>(entity) {
                    transform.x += dx;
                    transform.y += dy;
                }
            } else {
                systems::input::update_hover_system(&mut self.world, nx, ny);
            }
        }

        // 2. Process Mouse Down
        if self.mouse_down_triggered {
            self.mouse_down_triggered = false;
            
            // Apply selection logic
            systems::input::update_selection_system(&mut self.world);
            
            // Find drag target in a separate borrow
            self.drag_target = self.world.query_mut::<&Selectable>()
                .into_iter()
                .find(|(_, s)| s.is_hovered)
                .map(|(e, _)| e);
        }

        // 3. Process Mouse Up
        if self.mouse_up_triggered {
            self.mouse_up_triggered = false;
            self.drag_target = None;
        }
    }

    // These methods now ONLY update internal state, 
    // they never touch the World directly. This prevents aliasing.
    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        self.next_mouse_pos = Some((x, y));
    }

    pub fn handle_mouse_down(&mut self) {
        self.mouse_down_triggered = true;
    }

    pub fn handle_mouse_up(&mut self) {
        self.mouse_up_triggered = true;
    }



    pub fn apply_command(&mut self, command: Box<dyn commands::Command>) {
        self.history.apply(&mut self.world, command);
    }
}

impl Default for SokuEngine {
    fn default() -> Self {
        Self::new()
    }
}
