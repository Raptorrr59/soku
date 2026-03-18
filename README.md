# Project 2: **"Soku" (即) Collaborative Vector Engine**
*A high-performance, real-time collaborative whiteboard powered by a Rust-to-WebAssembly engine.*

## Vision
Soku is a professional-grade vector drawing engine designed for extreme performance and seamless multi-user collaboration. By offloading the complex geometry and state management to a specialized Rust engine running in WebAssembly, Soku achieves 60fps interactions even with thousands of objects on a shared canvas.

## High-Level Architecture
- **Rust Engine (`soku-core`):** Handles spatial indexing (R-Tree), scene graph management, the command pattern (undo/redo), and geometry math.
- **Wasm-JS Bridge:** Employs a zero-copy strategy via shared memory buffers and `wasm-bindgen` for high-speed communication between Rust and TypeScript.
- **Real-time Synchronization:** A WebSocket relay server manages "Active Sessions" and ensures state reconciliation across users.
- **React UI Layer:** Uses HTML5 Canvas for high-performance rendering of the main drawing area and SVG for high-fidelity UI elements.

---
For detailed technical specifications, roadmap, and learning objectives, see [PROJECT.md](./PROJECT.md).
