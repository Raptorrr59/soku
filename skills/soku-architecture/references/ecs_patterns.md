# Soku ECS Patterns

Soku uses a lightweight custom ECS or an established crate like `hecs` or `bevy_ecs`.

## Components (Pure Data)

Components must derive common traits (`Debug`, `Clone`, `Serialize`, `Deserialize`) and contain no methods other than simple constructors or getters.

```rust
// soku_core/src/components/transform.rs
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub scale: f32,
}

// soku_core/src/components/geometry.rs
#[derive(Debug, Clone)]
pub enum Shape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
    Path { points: Vec<(f32, f32)> },
}
```

## Systems (Pure Logic)

Systems should be isolated functions that take a query of the ECS world and mutate the components.

```rust
// soku_core/src/systems/physics.rs
pub fn apply_movement_system(world: &mut World) {
    // Example using a generic ECS query
    for (_id, (transform, velocity)) in world.query_mut::<(&mut Transform, &Velocity)>() {
        transform.x += velocity.dx;
        transform.y += velocity.dy;
    }
}
```

## Commands (Mutations)

To support Undo/Redo, user actions are not applied directly. They are dispatched as Commands.

```rust
pub enum Command {
    MoveEntity { entity_id: usize, dx: f32, dy: f32 },
    CreateShape { shape: Shape, transform: Transform },
    DeleteEntity { entity_id: usize },
}
```