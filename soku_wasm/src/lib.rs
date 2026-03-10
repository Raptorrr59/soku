use wasm_bindgen::prelude::*;
use soku_core::SokuEngine as CoreEngine;

#[wasm_bindgen]
pub struct SokuEngine {
    core: CoreEngine,
    /// A flat buffer of f32 values representing the render state.
    /// Stride (elements per shape): [ID, Type, X, Y, Width, Height, ...]
    render_buffer: Vec<f32>,
}

#[wasm_bindgen]
impl SokuEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SokuEngine {
        // Set up panic hook for better error messages in the browser console
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        SokuEngine {
            core: CoreEngine::new(),
            render_buffer: Vec::new(),
        }
    }

    /// Triggers an update of the internal render buffer based on the current ECS state.
    pub fn update_render_buffer(&mut self) {
        soku_core::systems::render::pack_render_buffer(&self.core.world, &mut self.render_buffer);
    }

    /// Returns the raw memory address of the render buffer.
    pub fn render_buffer_ptr(&self) -> *const f32 {
        self.render_buffer.as_ptr()
    }

    /// Returns the number of f32 elements in the render buffer.
    pub fn render_buffer_len(&self) -> usize {
        self.render_buffer.len()
    }
}

impl Default for SokuEngine {
    fn default() -> Self {
        Self::new()
    }
}
