use hecs::World;

pub trait Command {
    fn execute(&self, world: &mut World);
    fn undo(&self, world: &mut World);
}

pub struct HistoryManager {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}

impl HistoryManager {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn apply(&mut self, world: &mut World, command: Box<dyn Command>) {
        command.execute(world);
        self.undo_stack.push(command);
        // Once a new command is executed, redo history is invalidated
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, world: &mut World) {
        if let Some(command) = self.undo_stack.pop() {
            command.undo(world);
            self.redo_stack.push(command);
        }
    }

    pub fn redo(&mut self, world: &mut World) {
        if let Some(command) = self.redo_stack.pop() {
            command.execute(world);
            self.undo_stack.push(command);
        }
    }
}
