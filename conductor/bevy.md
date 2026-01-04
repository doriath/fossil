# Bevy 0.17 Learnings & Patterns

This document tracks key learnings, breaking changes, and best practices discovered while working with Bevy 0.17.

## 1. UI Construction (Component-Based)

Bevy 0.17 moves away from `*Bundle` types for UI construction in favor of individual components.

*   **Removed/Deprecated:** `NodeBundle`, `ButtonBundle`, `TextBundle`, `ImageBundle`.
*   **New Pattern:** Spawn entities with individual components.

### Example: Node & Button
```rust
commands.spawn((
    // Layout properties are now direct fields of Node, not Style
    Node {
        width: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    // Background color is a separate component
    BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
    // Button logic marker
    Button,
));
```

### Example: Text
```rust
commands.spawn((
    Text::new("Hello Bevy"),
    TextFont {
        font_size: 20.0,
        ..default()
    },
    TextColor(Color::WHITE),
));
```

## 2. Entity Despawning

*   **Changed:** `commands.entity(e).despawn_recursive()` is removed.
*   **New Pattern:** `commands.entity(e).despawn()` now handles the despawning (check migration guides for specifics on children, but `despawn()` is often the replacement).

## 3. Color API

*   **Changed:** `Color::rgba` is replaced by `Color::srgba`. Bevy 0.17 uses `Srgba` and `LinearRgba` more explicitly.

## 4. Testing Systems & State

*   **State Transitions:** When testing systems with `run_if(in_state(AppState::X))`, ensure you call `app.update()` enough times for the state transition to fully apply *before* the system is expected to run.
    ```rust
    app.world_mut().resource_mut::<NextState<AppState>>().set(AppState::InGame);
    app.update(); // Apply transition
    app.update(); // Settle state (sometimes needed before input/systems align)
    ```
*   **Input in Tests:** `Input<KeyCode>` updates frame-by-frame. `just_pressed` requires precise timing relative to `app.update()`. Ensure input is simulated *after* the state is settled if the system depends on both.
