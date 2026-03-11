import init, { SokuEngine } from "../wasm/soku_wasm";

export class SokuClient {
  private engine: SokuEngine | null = null;
  private wasmMemory: WebAssembly.Memory | null = null;
  private isInitialized = false;

  async init(): Promise<void> {
    if (this.isInitialized) return;
    
    // Initialize the Wasm module
    const wasm = await init();
    this.wasmMemory = wasm.memory;
    this.engine = new SokuEngine();
    this.isInitialized = true;
    
    console.log("Soku Wasm Engine Initialized successfully.");
  }

  update(): void {
    if (!this.engine) return;
    // Tell Rust to populate the render buffer
    this.engine.update_render_buffer();
  }

  handleMouseMove(x: number, y: number): void {
    this.engine?.handle_mouse_move(x, y);
  }

  handleMouseDown(): void {
    this.engine?.handle_mouse_down();
  }

  getRenderData(): Float32Array | null {
    if (!this.engine || !this.wasmMemory) return null;

    const ptr = this.engine.render_buffer_ptr();
    const len = this.engine.render_buffer_len();

    if (len === 0) return null;

    // Zero-Copy Memory Strategy:
    // Create a TypedArray view directly over the WebAssembly memory buffer.
    // ptr is a byte offset, so divide by 4 because each f32 is 4 bytes.
    return new Float32Array(this.wasmMemory.buffer, ptr, len);
  }
}
