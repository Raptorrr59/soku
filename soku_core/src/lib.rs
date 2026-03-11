pub mod commands;
pub mod components;
pub mod systems;

use hecs::World;
use commands::HistoryManager;
use components::{Shape, Transform, Selectable};

/// The central state of the Soku engine
pub struct SokuEngine {
    pub world: World,
    pub history: HistoryManager,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

impl SokuEngine {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Spawn some initial shapes to show off the ECS!
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
        }
    }

    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
        systems::input::update_hover_system(&mut self.world, x, y);
    }

    pub fn handle_mouse_down(&mut self) {
        systems::input::update_selection_system(&mut self.world);
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
