use crate::states::AppState;
use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            pause_game_on_escape.run_if(in_state(AppState::InGame)),
        );
    }
}

fn pause_game_on_escape(
    mut next_state: ResMut<NextState<AppState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::InputPlugin;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn test_can_transition_to_paused() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin).init_state::<AppState>();

        let mut next_state = app.world_mut().resource_mut::<NextState<AppState>>();
        next_state.set(AppState::Paused);

        app.update();
        app.update();

        let state = app.world().resource::<State<AppState>>().get();
        assert_eq!(state, &AppState::Paused);
    }

    #[test]
    fn test_escape_pauses_game() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(PauseMenuPlugin) // Add PauseMenuPlugin here
            .add_plugins(InputPlugin); // For KeyCode

        // Start in InGame state
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update(); // Apply state transition

        // Simulate Escape key press
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);

        // Run app update to process input and state change
        app.update();
        app.update();

        // Assert that state transitioned to Paused
        let state = app.world().resource::<State<AppState>>().get();
        assert_eq!(
            state,
            &AppState::Paused,
            "Game should pause when Escape is pressed"
        );
    }
}
