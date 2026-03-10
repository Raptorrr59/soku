pub mod commands;
pub mod components;
pub mod systems;

use hecs::World;
use commands::HistoryManager;
use components::{Shape, Transform};

/// The central state of the Soku engine
pub struct SokuEngine {
    pub world: World,
    pub history: HistoryManager,
}

impl SokuEngine {
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Spawn some initial shapes to show off the ECS!
        world.spawn((
            Transform { x: 100.0, y: 150.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            Shape::Rectangle { width: 200.0, height: 100.0 },
        ));
        
        world.spawn((
            Transform { x: 400.0, y: 300.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            Shape::Circle { radius: 80.0 },
        ));
        
        world.spawn((
            Transform { x: 600.0, y: 100.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            Shape::Rectangle { width: 120.0, height: 120.0 },
        ));

        world.spawn((
            Transform { x: 200.0, y: 400.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 },
            Shape::Circle { radius: 50.0 },
        ));

        Self {
            world,
            history: HistoryManager::new(),
        }
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
