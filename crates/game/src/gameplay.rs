use bevy::prelude::*;
use crate::states::AppState;
use bevy::camera::Camera2d;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_gameplay);
    }
}

fn setup_gameplay(mut commands: Commands) {
    // Placeholder for gameplay setup
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
            .add_plugins(GameplayPlugin);

        let mut next_state = app.world_mut().resource_mut::<NextState<AppState>>();
        next_state.set(AppState::InGame);

        app.update();
        app.update();

        let state = app.world().resource::<State<AppState>>().get();
        assert_eq!(state, &AppState::InGame);
    }

    #[test]
    fn test_camera_exists_in_gameplay() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(GameplayPlugin);
            
        // Manually spawn a camera as we would in main setup
        app.world_mut().spawn(Camera2d::default());

        let mut next_state = app.world_mut().resource_mut::<NextState<AppState>>();
        next_state.set(AppState::InGame);

        app.update();
        app.update();

        let count = app.world_mut().query::<&Camera2d>().iter(app.world()).len();
        assert_eq!(count, 1, "There should be one camera in gameplay");
    }
}
