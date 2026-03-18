# Soku Strict File Structure

The workspace must follow this exact structure to guarantee strict boundaries.

```text
projects/soku/
├── Cargo.toml (Workspace definition)
│
├── soku_core/                 # THE DOMAIN (Pure Rust, NO WASM, NO JS)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── components/        # Pure data structs
│       │   ├── mod.rs
│       │   ├── transform.rs
│       │   └── geometry.rs
│       ├── systems/           # Logic functions iterating over components
│       │   ├── mod.rs
│       │   └── render.rs
│       └── commands/          # Undo/Redo actions
│           └── mod.rs
│
├── soku_wasm/                 # THE ADAPTER (Wasm-bindgen lives here)
│   ├── Cargo.toml             # Depends on `soku_core`
│   └── src/
│       ├── lib.rs             # #[wasm_bindgen] definitions
│       ├── bridge.rs          # JS <-> Rust type conversions
│       └── memory.rs          # Zero-copy buffer management
│
└── soku_ui/                   # THE VIEW (React/TypeScript)
    ├── package.json
    ├── tsconfig.json
    └── src/
        ├── components/        # React components (Toolbar, Canvas)
        ├── hooks/             # useWasmEngine()
        └── engine/            # TypeScript wrappers around the Wasm binary
```

## Rules:
1. `soku_core` must NEVER have `wasm-bindgen` or `web-sys` in its `Cargo.toml`.
2. `soku_ui` must NEVER contain complex math or spatial indexing logic; it must query the Wasm engine for this data.
3. If code doesn't clearly fit into `components`, `systems`, or the `wasm` bridge, rethink the architecture before creating a new folder.