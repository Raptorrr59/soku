# Project 2: **"Soku" (即) Collaborative Vector Engine** - Project Details & Roadmap

## 1. Technical Specifications

### A. The Rust Engine (`soku-core`)
*   **Spatial Indexing (R-Tree / QuadTree):** Instead of iterating through every object on the canvas for every mouse event, the engine uses a spatial data structure to query objects in $O(\log n)$ time. This enables instant collision detection and selection.
*   **Scene Graph Management:** A robust tree structure representing the canvas state. Each node (path, rectangle, group) has a unique stable ID for multi-user synchronization.
*   **The Command Pattern:** Every mutation (Move, Delete, Create) is encapsulated in a Command object. This powers:
    *   **Undo/Redo History:** A stack-based system with a configurable memory limit.
    *   **Transactional Updates:** Changes are only committed to the main scene graph after successful validation.
*   **Geometry Math:** High-performance Bezier curve flattening, bounding box calculations, and affine transformations (translate, rotate, scale).

### B. The Wasm-JS Bridge
*   **Zero-Copy Strategy:** Instead of serializing large JSON objects between JS and Rust, Soku uses a shared `WebAssembly.Memory` buffer.
    *   Rust writes object positions/types into a fixed buffer.
    *   The TypeScript UI reads directly from this buffer via typed arrays (`Float32Array`).
*   **Wasm-bindgen:** Used for defining the high-level API exported to React (e.g., `engine.add_shape(type, x, y)`).
*   **Serialization:** Using `bincode` or `MessagePack` for complex state snapshots to keep the binary size small and parsing fast.

### C. Real-time Synchronization
*   **Centralized Authority (Single-Leader):** A lightweight WebSocket relay server (written in Rust using `Axum` or Go) that manages "Active Sessions."
*   **State Reconciliation:** When a user receives a remote update, the Rust engine calculates the diff and applies it to the local scene graph.
*   **Conflict Resolution:** Last-Write-Wins (LWW) per object attribute to prevent visual inconsistencies.

### D. The React UI Layer
*   **Canvas vs. SVG Rendering:**
    *   **HTML5 Canvas:** Used for the main drawing area to ensure high performance with thousands of objects.
    *   **SVG Overlay:** Used for high-fidelity UI elements like selection handles and text inputs.
*   **Optimized Re-renders:** React only manages the "Shell" (toolbars, layers list). The actual canvas drawing is triggered by the Rust engine via a `requestAnimationFrame` loop.

---

## 2. Highly Detailed Objectives

### Phase 1: The Engine & Wasm Bridge (Week 1-2)
*   [ ] **Geometry Library:** Implement basic shapes (Rect, Circle, Line) and bounding box logic in Rust.
*   [ ] **Spatial Index:** Integrate an R-Tree crate for fast object picking.
*   [ ] **Wasm Pipeline:** Set up `wasm-pack` with a basic React/TypeScript project.
*   [ ] **Buffer Rendering:** Successfully draw 10,000 shapes from Rust memory onto a JS `<canvas>` at 60fps.

### Phase 2: Interactivity & State (Week 3-4)
*   [ ] **Transformation Engine:** Implement dragging, resizing, and rotation handles in Rust logic.
*   [ ] **The Undo/Redo Stack:** Create a robust Command-based history system.
*   [ ] **Advanced Geometry:** Implement Pen tool support (Bezier curves) with path simplification algorithms.
*   [ ] **Serialization:** Implement Save/Load functionality using a binary format.

### Phase 3: Collaboration & Polish (Week 5-7)
*   [ ] **Relay Server:** Build a high-performance WebSocket server to broadcast binary patches.
*   [ ] **Multi-user Cursors:** Implement low-latency visual feedback for other users' mouse positions.
*   [ ] **Snapshot Sync:** Handle "late joiners" by sending them the full current state snapshot from the server.
*   [ ] **Export System:** Implement high-quality SVG/PDF export using Rust's `resvg` or similar crates.

---

## 3. What You Will Learn (Deep Dive)
*   **Manual Memory Management:** Managing the heap across the JS/Wasm boundary.
*   **Computational Geometry:** Spatial data structures and math for vector manipulation.
*   **Real-time Networking:** Optimizing WebSocket payloads and handling network jitter in interactive apps.
*   **Hybrid Architecture:** Designing systems where performance-critical logic is strictly separated from the UI layer.
