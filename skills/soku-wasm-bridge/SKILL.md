---
name: soku-wasm-bridge
description: Defines the Zero-Copy memory sharing strategies and the Wasm-bindgen interface for the Soku project. Use this when writing code that passes data between the Rust engine and the TypeScript frontend.
---

# Soku Wasm Bridge Guidelines

The primary performance bottleneck in any WebAssembly application is the serialization/deserialization of data between JavaScript and Rust (e.g., passing large JSON strings). Soku bypasses this completely by using a **Zero-Copy Memory Strategy**.

## Core Mandates

### 1. Zero-Copy over Serialization
Never use `serde_json` to pass large arrays of geometry data (like positions of thousands of shapes) to the frontend.
Instead, Rust must write to a flat `Vec<f32>` or `Vec<u8>`, and expose a pointer to that vector. TypeScript will read this directly via `WebAssembly.Memory`.

See [zero_copy.md](references/zero_copy.md) for the exact implementation pattern.

### 2. The `Engine` Interface
The `soku_wasm` crate must expose a single, cohesive `Engine` class to JavaScript via `#[wasm_bindgen]`.
This struct holds the state (the ECS World) and exposes methods to modify it.

```rust
// In soku_wasm/src/lib.rs
#[wasm_bindgen]
pub struct SokuEngine {
    world: World,
    render_buffer: Vec<f32>,
}

#[wasm_bindgen]
impl SokuEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SokuEngine { ... }

    pub fn dispatch_command(&mut self, command_type: u8, payload: &[u8]) { ... }
    
    pub fn get_render_buffer_ptr(&self) -> *const f32 {
        self.render_buffer.as_ptr()
    }
}
```

### 3. TypeScript Wrappers
Never call `wasm_bindgen` functions directly from React UI components. Always wrap the Wasm calls in a TypeScript class (`SokuClient`) located in `soku_ui/src/engine/` that handles the pointer arithmetic and `Float32Array` views.
