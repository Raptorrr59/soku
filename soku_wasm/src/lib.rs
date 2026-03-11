use wasm_bindgen::prelude::*;
use soku_core::SokuEngine as CoreEngine;

use std::sync::Mutex;
use once_cell::sync::Lazy;

struct GlobalInput {
    mouse_x: f32,
    mouse_y: f32,
    mouse_down: bool,
    mouse_up: bool,
}

static INPUT: Lazy<Mutex<GlobalInput>> = Lazy::new(|| Mutex::new(GlobalInput {
    mouse_x: 0.0,
    mouse_y: 0.0,
    mouse_down: false,
    mouse_up: false,
}));

#[wasm_bindgen]
pub struct SokuEngine {
    core: CoreEngine,
    render_buffer: Vec<f32>,
}

#[wasm_bindgen]
impl SokuEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SokuEngine {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        SokuEngine {
            core: CoreEngine::new(),
            render_buffer: Vec::new(),
        }
    }

    pub fn step(&mut self) {
        // Pull input from global state safely
        if let Ok(mut input) = INPUT.lock() {
            self.core.handle_mouse_move(input.mouse_x, input.mouse_y);
            
            if input.mouse_down {
                self.core.handle_mouse_down();
                input.mouse_down = false;
            }
            
            if input.mouse_up {
                self.core.handle_mouse_up();
                input.mouse_up = false;
            }
        }
        self.core.step();
    }

    pub fn update_render_buffer(&mut self) {
        soku_core::systems::render::pack_render_buffer(&self.core.world, &mut self.render_buffer);
    }

    pub fn handle_mouse_move(&self, x: f32, y: f32) {
        if let Ok(mut input) = INPUT.lock() {
            input.mouse_x = x;
            input.mouse_y = y;
        }
    }

    pub fn handle_mouse_down(&self) {
        if let Ok(mut input) = INPUT.lock() {
            input.mouse_down = true;
        }
    }

    pub fn handle_mouse_up(&self) {
        if let Ok(mut input) = INPUT.lock() {
            input.mouse_up = true;
        }
    }

    pub fn render_buffer_ptr(&self) -> *const f32 {
        self.render_buffer.as_ptr()
    }

    pub fn render_buffer_len(&self) -> usize {
        self.render_buffer.len()
    }
}

impl Default for SokuEngine {
    fn default() -> Self {
        Self::new()
    }
}
