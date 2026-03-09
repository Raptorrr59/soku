pub mod commands;
pub mod components;
pub mod systems;

use hecs::World;
use commands::HistoryManager;

/// The central state of the Soku engine
pub struct SokuEngine {
    pub world: World,
    pub history: HistoryManager,
}

impl SokuEngine {
    pub fn new() -> Self {
        Self {
            world: World::new(),
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
