---
name: soku-rust-core
description: Defines the strict rules for the Soku core math and geometry engine. Use this when writing spatial algorithms, collision detection, or history management in the soku_core crate.
---

# Soku Core Engine Guidelines

The `soku_core` crate is the heart of the application. It must be highly optimized, mathematically rigorous, and completely platform-agnostic (no DOM, no Wasm, no Network logic).

## Core Mandates

### 1. Spatial Indexing is Mandatory
Never iterate through all entities to find which one was clicked. You MUST use an R-Tree (via crates like `rstar` or `spade`) to manage spatial relationships.
When an entity moves, its bounding box in the R-Tree must be updated immediately in the same frame.

### 2. The Command Pattern (Undo/Redo)
Directly mutating state from user input is forbidden. All user intents must be serialized into a `Command`.
The engine maintains a History Stack (`Vec<Command>`).

To apply a change:
1. Construct the `Command`.
2. Push to the Undo stack.
3. Call `command.execute(&mut world)`.

See [commands.md](references/commands.md) for the implementation details.

### 3. Determinism
The core engine must be 100% deterministic. Given the exact same sequence of Commands, the resulting ECS World state must be identical down to the bit. Do not rely on system time, random number generators (without fixed seeds), or floating-point non-determinism across platforms.