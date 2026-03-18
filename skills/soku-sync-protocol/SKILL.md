---
name: soku-sync-protocol
description: Defines the real-time multiplayer synchronization protocols for Soku. Use this when implementing WebSocket payloads, state reconciliation, or conflict resolution logic.
---

# Soku Synchronization Protocol

Soku is a collaborative whiteboard. To maintain high performance and low latency, it relies on a binary patching protocol rather than sending full state snapshots or heavy JSON objects.

## Core Mandates

### 1. Binary Payloads
WebSocket messages MUST use a compact binary serialization format (like `bincode` or `MessagePack`). Do not use JSON for high-frequency sync events like cursor movements or object dragging.

### 2. State Reconciliation (Single-Leader)
Soku uses an authoritative server model. 
- The server (written in Rust or Go) holds the "True State".
- Clients send `Intent` commands to the server.
- The server validates the command, applies it, and broadcasts the resulting `Patch` to all connected clients.
- Clients apply incoming patches to their local ECS world.

See [reconciliation.md](references/reconciliation.md) for handling latency and local prediction.

### 3. High-Frequency vs Low-Frequency Sync
Separate the data streams:
- **High-Frequency (Cursors, Dragging)**: Sent unreliably (UDP-style or volatile WebSockets). Dropped packets don't matter because a newer position is always coming.
- **Low-Frequency (Create, Delete, Undo)**: Sent reliably. Must be acknowledged by the server.