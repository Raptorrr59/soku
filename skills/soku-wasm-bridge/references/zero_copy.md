# Zero-Copy Memory Sharing

To achieve 60fps with thousands of shapes, Soku uses shared memory.

## The Rust Side
Rust maintains a contiguous block of memory (`Vec<f32>`). During the render phase, a System iterates over all `Renderable` and `Transform` components and writes their data into this flat array in a strict format.

Example format per shape (Stride = 6 floats):
`[ID, Type, X, Y, Width, Height]`

```rust
#[wasm_bindgen]
impl SokuEngine {
    // Returns the memory address of the first element
    pub fn render_buffer_ptr(&self) -> *const f32 {
        self.render_buffer.as_ptr()
    }

    // Returns the number of floats currently in the buffer
    pub fn render_buffer_len(&self) -> usize {
        self.render_buffer.len()
    }
}
```

## The TypeScript Side
TypeScript uses these pointers to create a `Float32Array` view over the WebAssembly memory. This operation is instantaneous and requires no data copying.

```typescript
// soku_ui/src/engine/SokuClient.ts
import { memory } from "soku_wasm/soku_wasm_bg.wasm";

class SokuClient {
  public getRenderData(): Float32Array {
    const ptr = this.engine.render_buffer_ptr();
    const len = this.engine.render_buffer_len();
    
    // Create a view over the shared memory. 
    // Note: ptr is a byte offset, so divide by 4 for Float32Array
    return new Float32Array(memory.buffer, ptr, len);
  }
}
```

**CRITICAL WARNING:** The `Float32Array` view becomes detached and invalid if WebAssembly memory grows (e.g., when a new large `Vec` is allocated in Rust). Always recreate the `Float32Array` view every single frame before reading from it.