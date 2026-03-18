use wasm_bindgen::prelude::*;
use soku_core::SokuEngine as CoreEngine;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// Permanent Rust state
static ENGINE: Lazy<Mutex<CoreEngine>> = Lazy::new(|| Mutex::new(CoreEngine::new()));
static RENDER_BUFFER: Lazy<Mutex<Vec<f32>>> = Lazy::new(|| Mutex::new(Vec::with_capacity(2048)));

// --- TOP-LEVEL WASM API ---
// We use functions instead of a struct to bypass wasm-bindgen's aggressive aliasing locks.
// Our internal Rust Mutexes will handle the actual synchronization.

#[wasm_bindgen]
pub fn soku_init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn soku_step(mouse_x: f32, mouse_y: f32, mouse_down: bool, mouse_up: bool) {
    if let Ok(mut engine) = ENGINE.lock() {
        engine.step(mouse_x, mouse_y, mouse_down, mouse_up);
    }
}

#[wasm_bindgen]
pub fn soku_update_render_buffer(min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
    if let (Ok(engine), Ok(mut buffer)) = (ENGINE.lock(), RENDER_BUFFER.lock()) {
        engine.render(&mut buffer, min_x, min_y, max_x, max_y);
    }
}

#[wasm_bindgen]
pub fn soku_spawn_shape(shape_type: u8, x: f32, y: f32) {
    if let Ok(mut engine) = ENGINE.lock() {
        engine.spawn_shape(shape_type, x, y);
    }
}

#[wasm_bindgen]
pub fn soku_update_selected_color(color_hex: String) {
    if let Ok(mut engine) = ENGINE.lock() {
        engine.update_selected_color(&color_hex);
    }
}

#[wasm_bindgen]
pub fn soku_update_selected_zindex(delta: f32) {
    if let Ok(mut engine) = ENGINE.lock() {
        engine.update_selected_zindex(delta);
    }
}

#[wasm_bindgen]
pub fn soku_resize_selected(factor: f32) {
    if let Ok(mut engine) = ENGINE.lock() {
        engine.resize_selected(factor);
    }
}

#[wasm_bindgen]
pub fn soku_delete_selected() {
    if let Ok(mut engine) = ENGINE.lock() {
        engine.delete_selected();
    }
}

#[wasm_bindgen]
pub fn soku_move_camera(dx: f32, dy: f32) {
    if let Ok(mut engine) = ENGINE.lock() {
        engine.move_camera(dx, dy);
    }
}

#[wasm_bindgen]
pub fn soku_zoom_camera(delta: f32) {
    if let Ok(mut engine) = ENGINE.lock() {
        engine.zoom_camera(delta);
    }
}

#[wasm_bindgen]
pub fn soku_get_camera_x() -> f32 {
    if let Ok(engine) = ENGINE.lock() {
        engine.get_camera().x
    } else {
        0.0
    }
}

#[wasm_bindgen]
pub fn soku_get_camera_y() -> f32 {
    if let Ok(engine) = ENGINE.lock() {
        engine.get_camera().y
    } else {
        0.0
    }
}

#[wasm_bindgen]
pub fn soku_get_camera_zoom() -> f32 {
    if let Ok(engine) = ENGINE.lock() {
        engine.get_camera().zoom
    } else {
        1.0
    }
}

#[wasm_bindgen]
pub fn soku_render_buffer_ptr() -> *const f32 {
    if let Ok(buffer) = RENDER_BUFFER.lock() {
        buffer.as_ptr()
    } else {
        std::ptr::null()
    }
}

#[wasm_bindgen]
pub fn soku_render_buffer_len() -> usize {
    if let Ok(buffer) = RENDER_BUFFER.lock() {
        buffer.len()
    } else {
        0
    }
}
