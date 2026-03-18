# Command Pattern Implementation

Every change to the canvas must be a reversible command. This allows for both local Undo/Redo and remote multi-user sync.

## Defining Commands

Commands should contain exactly the data needed to apply the change, and the inverse data needed to revert it.

```rust
// soku_core/src/commands/mod.rs

pub trait Command {
    fn execute(&self, world: &mut World);
    fn undo(&self, world: &mut World);
}

pub struct MoveCommand {
    pub entity_id: usize,
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
}

impl Command for MoveCommand {
    fn execute(&self, world: &mut World) {
        if let Ok(mut transform) = world.get_mut::<Transform>(self.entity_id) {
            transform.x = self.end_x;
            transform.y = self.end_y;
        }
    }

    fn undo(&self, world: &mut World) {
        if let Ok(mut transform) = world.get_mut::<Transform>(self.entity_id) {
            transform.x = self.start_x;
            transform.y = self.start_y;
        }
    }
}
```

## History Manager

The `soku_core` engine should wrap the world in a manager that handles the stacks.

```rust
pub struct HistoryManager {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}
```