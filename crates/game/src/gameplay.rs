use bevy::prelude::*;
use crate::states::AppState;
use bevy::camera::Camera2d;

#[derive(Component)]
pub struct Player;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_gameplay);
    }
}

fn setup_gameplay(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
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
            .insert_resource(ClearColor::default());

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
            .insert_resource(ClearColor::default());

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
            .insert_resource(ClearColor::default());
            
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
            .insert_resource(ClearColor(Color::BLACK)); // Default

        let mut next_state = app.world_mut().resource_mut::<NextState<AppState>>();
        next_state.set(AppState::InGame);

        app.update();
        app.update();

        let clear_color = app.world().resource::<ClearColor>();
        assert_eq!(clear_color.0, Color::srgb(0.0, 1.0, 0.0), "Background should be green in gameplay");
    }
}
