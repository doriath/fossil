use crate::states::AppState;
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u64);

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_gameplay)
            .add_systems(Update, move_player.run_if(in_state(AppState::InGame)));
    }
}

fn setup_gameplay(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    player_query: Query<Entity, With<Player>>,
) {
    // Only spawn player if one doesn't already exist
    if player_query.iter().next().is_none() {
        clear_color.0 = Color::srgb(0.0, 1.0, 0.0);

        // Spawn Player
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Player,
        ));
    }
}

fn move_player(
    mut transforms: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 100.0;
    for mut transform in &mut transforms {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * speed * time.delta_secs();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn test_can_transition_to_ingame() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(GameplayPlugin)
            .insert_resource(ClearColor::default())
            .add_plugins(bevy::input::InputPlugin)
            .add_plugins(bevy::time::TimePlugin);

        let mut next_state = app.world_mut().resource_mut::<NextState<AppState>>();
        next_state.set(AppState::InGame);

        app.update();
        app.update();

        let state = app.world().resource::<State<AppState>>().get();
        assert_eq!(state, &AppState::InGame);
    }

    #[test]
    fn test_player_exists() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(GameplayPlugin)
            .insert_resource(ClearColor::default())
            .add_plugins(bevy::input::InputPlugin)
            .add_plugins(bevy::time::TimePlugin);

        let mut next_state = app.world_mut().resource_mut::<NextState<AppState>>();
        next_state.set(AppState::InGame);

        app.update();
        app.update();

        let count = app.world_mut().query::<&Player>().iter(app.world()).len();
        assert_eq!(count, 1, "Player should be spawned in InGame state");
    }

    #[test]
    fn test_camera_exists_in_gameplay() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(GameplayPlugin)
            .insert_resource(ClearColor::default())
            .add_plugins(bevy::input::InputPlugin)
            .add_plugins(bevy::time::TimePlugin);

        // Manually spawn a camera as we would in main setup
        app.world_mut().spawn(Camera2d::default());

        let mut next_state = app.world_mut().resource_mut::<NextState<AppState>>();
        next_state.set(AppState::InGame);

        app.update();
        app.update();

        let count = app.world_mut().query::<&Camera2d>().iter(app.world()).len();
        assert_eq!(count, 1, "There should be one camera in gameplay");
    }

    #[test]
    fn test_background_is_green() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(GameplayPlugin)
            .insert_resource(ClearColor(Color::BLACK)) // Default
            .add_plugins(bevy::input::InputPlugin)
            .add_plugins(bevy::time::TimePlugin);

        let mut next_state = app.world_mut().resource_mut::<NextState<AppState>>();
        next_state.set(AppState::InGame);

        app.update();
        app.update();

        let clear_color = app.world().resource::<ClearColor>();
        assert_eq!(
            clear_color.0,
            Color::srgb(0.0, 1.0, 0.0),
            "Background should be green in gameplay"
        );
    }

    #[test]
    fn test_player_moves_up_on_w_press() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(GameplayPlugin)
            .insert_resource(ClearColor::default())
            .add_plugins(bevy::input::InputPlugin) // Required for ButtonInput<KeyCode>
            .add_plugins(bevy::time::TimePlugin);

        // Set initial state to InGame
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update(); // Apply state transition

        // Get the player's initial position
        let initial_position = app
            .world_mut()
            .query_filtered::<&Transform, With<Player>>()
            .iter(app.world())
            .next()
            .map(|t| t.translation)
            .unwrap_or(Vec3::ZERO);

        // Simulate 'W' key press
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);

        // Run the app update to process input and movement system
        app.update(); // Run once to ensure systems run

        // Get the player's new position
        let new_position = app
            .world_mut()
            .query_filtered::<&Transform, With<Player>>()
            .iter(app.world())
            .next()
            .map(|t| t.translation)
            .unwrap_or(Vec3::ZERO);

        // Assert that the player's Y position has increased
        assert!(
            new_position.y > initial_position.y,
            "Player should move up when 'W' is pressed"
        );
    }

    #[test]
    fn test_resume_game_does_not_spawn_extra_player() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(GameplayPlugin)
            .insert_resource(ClearColor::default())
            .add_plugins(bevy::input::InputPlugin)
            .add_plugins(bevy::time::TimePlugin);

        // First transition to InGame to spawn initial player
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update();
        app.update();

        let initial_player_count = app.world_mut().query::<&Player>().iter(app.world()).len();
        assert_eq!(initial_player_count, 1, "Initial player count should be 1");

        // Now transition to Paused
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::Paused);
        app.update();
        app.update();

        // Then transition back to InGame (simulating resume)
        app.world_mut()
            .resource_mut::<NextState<AppState>>()
            .set(AppState::InGame);
        app.update();
        app.update();

        // Assert that no new player was spawned
        let final_player_count = app.world_mut().query::<&Player>().iter(app.world()).len();
        assert_eq!(
            final_player_count, 1,
            "Resuming game should not spawn an extra player"
        );
    }
}
