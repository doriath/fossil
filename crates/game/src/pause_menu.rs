use bevy::prelude::*;
use crate::states::AppState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, _app: &mut App) {
        // No systems yet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn test_can_transition_to_paused() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>();

        let mut next_state = app.world_mut().resource_mut::<NextState<AppState>>();
        next_state.set(AppState::Paused);

        app.update();
        app.update();

        let state = app.world().resource::<State<AppState>>().get();
        assert_eq!(state, &AppState::Paused);
    }
}
