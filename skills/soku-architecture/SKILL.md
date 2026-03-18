---
name: soku-architecture
description: Enforces Entity-Component-System (ECS) architecture and strict clean file boundaries for the Soku vector engine project (Rust + Wasm). Use this when writing or refactoring code in the Soku project to ensure architectural compliance.
---

# Soku Architecture Guidelines

This skill defines the architectural mandates for the Soku vector drawing engine. Soku is a high-performance Rust + Wasm application. To maintain performance and maintainability, it strictly follows the **Entity-Component-System (ECS)** pattern and maintains rigid folder boundaries.

## Core Architectural Mandates

### 1. Entity-Component-System (ECS)
Do not use traditional Object-Oriented inheritance (e.g., `struct Rectangle { shape: Shape }`).
- **Entities**: Are just unique IDs (e.g., `usize` or `uuid`).
- **Components**: Are pure data structures (structs/enums) with NO logic. (e.g., `Position { x: f32, y: f32 }`, `Renderable { color: String }`).
- **Systems**: Are pure functions that iterate over specific combinations of components. They contain ALL the logic (e.g., `fn render_system(query: Query<(&Position, &Renderable)>)`).

For detailed examples of how to write components and systems, see [ecs_patterns.md](references/ecs_patterns.md).

### 2. Strict Boundary Enforcement ("Nothing outside its box")
The codebase is divided into distinct crates or modules. Code from one box cannot leak into another.
- **Core Domain (`soku_core`)**: The pure Rust ECS engine. Knows absolutely nothing about Wasm, the DOM, or React. Uses `no_std` where possible, or strictly standard library.
- **Wasm Bridge (`soku_wasm`)**: The ONLY place where `wasm-bindgen` and `js-sys` are allowed. Translates JS calls into ECS commands and reads ECS state to pass back to JS via shared memory.
- **Frontend (`soku_ui`)**: The React/TypeScript app. Contains zero heavy geometry logic. Only handles user inputs, passes them to Wasm, and renders what Wasm tells it to render.

For the exact file tree and module rules, see [folder_structure.md](references/folder_structure.md).

## Workflow

When asked to add a feature to Soku (e.g., "Add a circle tool"):
1. **Define Components**: First, identify if new data structures are needed in `soku_core/src/components/`.
2. **Write Systems**: Create or update the logic in `soku_core/src/systems/`.
3. **Expose Bridge**: Update the Wasm API in `soku_wasm/src/lib.rs` to allow the frontend to trigger the new system.
4. **Update UI**: Finally, wire up the React component in `soku_ui` to call the Wasm bridge.
