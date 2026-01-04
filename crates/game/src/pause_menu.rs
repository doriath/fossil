use crate::states::AppState;
use bevy::prelude::*;


pub struct PauseMenuPlugin;

#[derive(Component)]
struct PauseMenuUi; // Marker component for the pause menu UI

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                pause_game_on_escape.run_if(in_state(AppState::InGame)),
                pause_menu_action.run_if(in_state(AppState::Paused)),
            ),
        )
        .add_systems(OnEnter(AppState::Paused), setup_pause_menu);
        // Add despawn system later
    }
}

fn pause_menu_action(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, children) in &interaction_query {
        if *interaction == Interaction::Pressed {
            for &child in children {
                if let Ok(text) = text_query.get(child) {
                    if text.0 == "Continue" {
                        next_state.set(AppState::InGame);
                    }
                }
            }
        }
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

// System to setup pause menu
fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            PauseMenuUi,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Paused"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Continue Button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Continue"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

            // Exit to Main Menu Button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(65.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Exit to Main Menu"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
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

    #[test]
    fn test_pause_menu_spawns_on_paused_state() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(PauseMenuPlugin); // PauseMenuPlugin will contain the setup system

        app.world_mut().resource_mut::<NextState<AppState>>().set(AppState::Paused);
        app.update();
        app.update();

        // Check for the presence of UI elements (e.g., specific Text components)
        let mut query = app.world_mut().query::<&Text>();
        let mut found_continue = false;
        let mut found_exit = false;

        for text in query.iter(app.world()) {
            if text.0 == "Continue" {
                found_continue = true;
            }
            if text.0 == "Exit to Main Menu" {
                found_exit = true;
            }
        }
        assert!(found_continue, "Continue button text not found in pause menu");
        assert!(found_exit, "Exit to Main Menu button text not found in pause menu");
    }

    #[test]
    fn test_continue_button_resumes_game() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_plugins(InputPlugin)
            .add_plugins(PauseMenuPlugin);

        // Start in Paused state
        app.world_mut().resource_mut::<NextState<AppState>>().set(AppState::Paused);
        app.update();
        app.update();

        // Find "Continue" button entity
        let mut continue_button_entity = None;
        // Text is a child of the Button. Query for Text and getting its Parent (ChildOf)
        let mut query = app.world_mut().query::<(&Text, &ChildOf)>();
        
        for (text, parent) in query.iter(app.world()) {
            if text.0 == "Continue" {
                continue_button_entity = Some(parent.parent());
                break;
            }
        }
        
        let continue_button_entity = continue_button_entity.expect("Continue button not found");

        // Simulate button press
        app.world_mut().entity_mut(continue_button_entity).insert(Interaction::Pressed);

        app.update();
        app.update(); // Process state transition

        let state = app.world().resource::<State<AppState>>().get();
        assert_eq!(state, &AppState::InGame, "Game should resume when Continue is pressed");
    }
}
