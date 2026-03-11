import init, { SokuEngine } from "../wasm/soku_wasm";

export class SokuClient {
  private engine: SokuEngine | null = null;
  private wasmModule: any = null;
  private isInitialized = false;

  async init(): Promise<void> {
    if (this.isInitialized) return;
    
    // Initialize the Wasm module
    const wasm = await init();
    // We store the whole wasm object so we can access wasm.memory.buffer 
    // which updates dynamically if memory grows.
    this.wasmModule = wasm;
    this.engine = new SokuEngine();
    this.isInitialized = true;
    
    console.log("Soku Wasm Engine Initialized successfully.");
  }

  update(): void {
    if (!this.engine) return;
    // Single point of entry for mutation
    this.engine.step();
    // Single point of entry for buffer packing
    this.engine.update_render_buffer();
  }

  handleMouseMove(x: number, y: number): void {
    this.engine?.handle_mouse_move(x, y);
  }

  handleMouseDown(): void {
    this.engine?.handle_mouse_down();
  }

  handleMouseUp(): void {
    this.engine?.handle_mouse_up();
  }

  getRenderData(): Float32Array | null {
    if (!this.engine || !this.wasmModule) return null;

    const ptr = this.engine.render_buffer_ptr();
    const len = this.engine.render_buffer_len();

    if (len === 0) return null;

    // Zero-Copy Memory Strategy:
    // ALWAYS access the buffer through this.wasmModule.memory.buffer
    // This ensures that if the memory grows, we get the new buffer instead of a detached one.
    return new Float32Array(this.wasmModule.memory.buffer, ptr, len);
  }
}
