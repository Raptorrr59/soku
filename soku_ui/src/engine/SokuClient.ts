import init, { 
  soku_init, 
  soku_step, 
  soku_update_render_buffer, 
  soku_spawn_shape, 
  soku_delete_selected, 
  soku_update_selected_color,
  soku_update_selected_zindex,
  soku_resize_selected,
  soku_move_camera,
  soku_zoom_camera,
  soku_get_camera_x,
  soku_get_camera_y,
  soku_get_camera_zoom,
  soku_render_buffer_ptr, 
  soku_render_buffer_len 
} from "../wasm/soku_wasm";

export class SokuClient {
  private wasmModule: any = null;
  private isInitialized = false;

  // Local input state (JS-only)
  private mouseX = 0;
  private mouseY = 0;
  private isMouseDown = false;
  private isMouseUp = false;

  // Action Queue (JS-only)
  private pendingSpawn: { type: number, x: number, y: number } | null = null;
  private pendingDelete = false;
  private pendingColorUpdate: string | null = null;
  private pendingZIndexUpdate: number | null = null;
  private pendingResize: number | null = null;

  async init(): Promise<void> {
    if (this.isInitialized) return;
    const wasm = await init();
    this.wasmModule = wasm;
    
    // Call global init
    soku_init();
    
    this.isInitialized = true;
    console.log("Soku Wasm Engine Initialized successfully.");
  }

  update(minX: number = -10000, minY: number = -10000, maxX: number = 10000, maxY: number = 10000): void {
    if (!this.isInitialized) return;
    
    // 1. Process pending structural changes
    if (this.pendingSpawn) {
      soku_spawn_shape(this.pendingSpawn.type, this.pendingSpawn.x, this.pendingSpawn.y);
      this.pendingSpawn = null;
    }

    if (this.pendingDelete) {
      soku_delete_selected();
      this.pendingDelete = false;
    }

    if (this.pendingColorUpdate) {
      soku_update_selected_color(this.pendingColorUpdate);
      this.pendingColorUpdate = null;
    }

    if (this.pendingZIndexUpdate !== null) {
      soku_update_selected_zindex(this.pendingZIndexUpdate);
      this.pendingZIndexUpdate = null;
    }

    if (this.pendingResize !== null) {
      soku_resize_selected(this.pendingResize);
      this.pendingResize = null;
    }

    // 2. Pass JS state to Rust via top-level function
    soku_step(this.mouseX, this.mouseY, this.isMouseDown, this.isMouseUp);
    
    // Reset triggers
    this.isMouseDown = false;
    this.isMouseUp = false;

    // 3. Update the render buffer with viewport culling
    soku_update_render_buffer(minX, minY, maxX, maxY);
  }

  handleMouseMove(x: number, y: number): void {
    this.mouseX = x;
    this.mouseY = y;
  }

  handleMouseDown(): void {
    this.isMouseDown = true;
  }

  handleMouseUp(): void {
    this.isMouseUp = true;
  }

  moveCamera(dx: number, dy: number): void {
    soku_move_camera(dx, dy);
  }

  zoomCamera(delta: number): void {
    soku_zoom_camera(delta);
  }

  getCamera(): { x: number, y: number, zoom: number } {
    return {
      x: soku_get_camera_x(),
      y: soku_get_camera_y(),
      zoom: soku_get_camera_zoom()
    };
  }

  spawnShape(type: number, x: number, y: number): void {
    this.pendingSpawn = { type, x, y };
  }

  deleteSelected(): void {
    this.pendingDelete = true;
  }

  updateSelectedColor(hex: string): void {
    this.pendingColorUpdate = hex;
  }

  updateSelectedZIndex(delta: number): void {
    this.pendingZIndexUpdate = delta;
  }

  resizeSelected(factor: number): void {
    this.pendingResize = factor;
  }

  getRenderData(): Float32Array | null {
    if (!this.isInitialized || !this.wasmModule) return null;
    
    // Read from the static RENDER_BUFFER via top-level functions
    const ptr = soku_render_buffer_ptr();
    const len = soku_render_buffer_len();
    
    if (len === 0) return null;
    
    return new Float32Array(this.wasmModule.memory.buffer, ptr as number, len);
  }
}
