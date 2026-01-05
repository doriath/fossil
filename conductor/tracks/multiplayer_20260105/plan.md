# Plan: Multiplayer/Networking Capability

## Phase 1: Initial Network Setup
*   **Goal:** Establish the foundational network infrastructure, integrate `bevy_replicon` and `bevy_replicon_renet`, and enable server/client mode selection via CLI arguments.

*   [x] Task: Add networking dependencies. 0dba776
    *   [ ] Sub-task: Update `crates/game/Cargo.toml` with `bevy_replicon` and `bevy_replicon_renet` dependencies.
    *   [ ] Sub-task: Add `clap` as a dependency for command-line argument parsing.
*   [x] Task: Implement command-line argument parsing. 290baff
    *   [ ] Sub-task: Modify `crates/game/src/main.rs` to parse `--server` or `--client` arguments using `clap`.
    *   [ ] Sub-task: Define an enum or struct to hold the parsed network mode.
*   [x] Task: Initialize network plugins based on mode. 05b5455
    *   [ ] Sub-task: In `crates/game/src/main.rs`, conditionalize Bevy plugin setup based on server/client mode.
    *   [ ] Sub-task: Configure `bevy_replicon`'s server and client plugins.
    *   [ ] Sub-task: Configure `bevy_replicon_renet`'s server and client transport.
*   [ ] Task: Conductor - User Manual Verification 'Initial Network Setup' (Protocol in workflow.md) [checkpoint: 0e44815]

## Phase 2: Player Entity and Position Replication
*   **Goal:** Enable player entities to be spawned on the server, replicated to clients, and their positions synchronized across all connected instances.

*   [ ] Task: Define replicable player components.
    *   [ ] Sub-task: Create or modify existing player components (e.g., `PlayerId`, `Transform`) to derive `Replicate` trait for `bevy_replicon`.
    *   [ ] Sub-task: Add `RepliconAppExt` to the appropriate `App` for replication setup.
*   [ ] Task: Implement server-side player management.
    *   [ ] Sub-task: Write a server system to spawn a player entity when a new client connects.
    *   [ ] Sub-task: Assign a unique identifier to each player (e.g., `PlayerId` component) and associate it with the client connection.
    *   [ ] Sub-task: Ensure player entities created by the server are properly marked for replication to clients.
*   [ ] Task: Implement client-side player entity handling.
    *   [ ] Sub-task: Write a client system to despawn player entities when a client disconnects.
    *   [ ] Sub-task: Write a client system to identify the local player versus remote players based on `PlayerId`.
    *   [ ] Sub-task: Add placeholder visual representation for remote players.
*   [ ] Task: Replicate player movement.
    *   [ ] Sub-task: Modify the local player movement system on the client to send movement input to the server.
    *   [ ] Sub-task: Implement a server system to receive client movement input and update the server-side player's `Transform`.
    *   [ ] Sub-task: Ensure the server's `Transform` updates are replicated back to all clients.
*   [ ] Task: Conductor - User Manual Verification 'Player Entity and Position Replication' (Protocol in workflow.md)

## Phase 3: End-to-End Testing and Refinement
*   **Goal:** Verify the full multiplayer flow, ensure accurate position synchronization, and perform any necessary performance or code refinements.

*   [ ] Task: Perform local multiplayer testing.
    *   [ ] Sub-task: Run the game in server mode.
    *   [ ] Sub-task: Run two separate game instances in client mode, connecting to the local server.
*   [ ] Task: Verify player position synchronization.
    *   [ ] Sub-task: Move the player character on Client 1 and visually confirm its movement on Client 2.
    *   [ ] Sub-task: Move the player character on Client 2 and visually confirm its movement on Client 1.
    *   [ ] Sub-task: Assess smoothness and responsiveness of replicated movement.
*   [ ] Task: Refactor and optimize.
    *   [ ] Sub-task: Review and refactor network-related code for clarity, efficiency, and adherence to Bevy best practices.
    *   [ ] Sub-task: Consider minor optimizations if performance issues are observed (e.g., update frequency).
*   [ ] Task: Conductor - User Manual Verification 'End-to-End Testing and Refinement' (Protocol in workflow.md)
