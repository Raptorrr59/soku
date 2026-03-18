# Client-Side Prediction and Reconciliation

To ensure the canvas feels instantaneous to the local user while waiting for the server to acknowledge an action, Soku uses Client-Side Prediction.

## The Flow

1. **User Action:** The user drags a shape.
2. **Local Application (Prediction):** The UI immediately applies the `MoveCommand` to the local ECS World. The shape moves instantly on screen.
3. **Dispatch Intent:** The client serializes the `MoveCommand` and sends it to the Server. The client adds this command to a `PendingUnacknowledged` list.
4. **Server Validation:** The server receives the command, validates it, applies it to the authoritative state, and broadcasts a `Patch` with a monotonically increasing `Sequence ID`.
5. **Reconciliation:** The client receives the `Patch`. 
   - It rolls back all commands in the `PendingUnacknowledged` list.
   - It applies the authoritative `Patch` from the server.
   - It re-applies any remaining commands in the `PendingUnacknowledged` list that the server hasn't seen yet.

This ensures the user never sees lag, but the server always has final say on conflicts (e.g., if two users try to grab the same object at the same time).