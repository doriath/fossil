mod states;
mod gameplay;
use crate::states::AppState;
use bevy::camera::Camera2d;
use bevy::prelude::*; // Added back explicit import

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(gameplay::GameplayPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
        .add_systems(Update, menu_action.run_if(in_state(AppState::MainMenu)))
        .run();
}

fn menu_action(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    text_query: Query<&Text>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    for (interaction, children) in &interaction_query {
        if *interaction == Interaction::Pressed {
            for &child in children {
                if let Ok(text) = text_query.get(child) {
                    if text.0 == "Start" {
                        next_state.set(AppState::InGame);
                    }
                    if text.0 == "Exit" {
                        app_exit_events.write(AppExit::Success);
                    }
                }
            }
        }
    }
}

fn main_menu_setup(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Main Menu"),
                TextFont {
                    font_size: 100.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));

            // Start Button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
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
                    Text::new("Start"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

            // Exit Button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
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
                    Text::new("Exit"),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::state::app::StatesPlugin;
    use states::AppState;

    #[test]
    fn test_app_has_app_state() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin);
        app.init_state::<AppState>(); // Initialize the AppState
        app.update(); // Run one frame for systems to execute
        assert!(app.world().get_resource::<State<AppState>>().is_some());
    }

    #[test]
    fn test_main_menu_title_is_present() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_systems(OnEnter(AppState::MainMenu), main_menu_setup);
        app.update(); // Run one frame for systems to execute

        // Check if the "Main Menu" text exists
        let mut query = app.world_mut().query::<&Text>();
        let mut found_text = false;
        for text in query.iter(&app.world()) {
            if text.0 == "Main Menu" {
                found_text = true;
                break;
            }
        }
        assert!(found_text, "Main Menu title text not found.");
    }

    #[test]
    fn test_main_menu_buttons_are_present() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_systems(OnEnter(AppState::MainMenu), main_menu_setup);
        app.update();

        let mut query = app.world_mut().query::<&Text>();
        let mut found_start = false;
        let mut found_exit = false;

        for text in query.iter(app.world()) {
            if text.0 == "Start" {
                found_start = true;
            }
            if text.0 == "Exit" {
                found_exit = true;
            }
        }
        assert!(found_start, "Start button text not found");
        assert!(found_exit, "Exit button text not found");
    }

    #[test]
    fn test_start_button_transitions_to_ingame() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
            .add_systems(Update, menu_action);
            // .add_systems(Update, menu_action.run_if(in_state(AppState::MainMenu))); 

        app.update();

        // Find Start button
        let mut start_button_entity = None;
        let mut query = app.world_mut().query::<(&Text, &ChildOf)>();
        
        for (text, parent) in query.iter(app.world()) {
            if text.0 == "Start" {
                start_button_entity = Some(parent.parent());
                break;
            }
        }
        
        let start_button_entity = start_button_entity.expect("Start button not found");

        // Simulate press
        app.world_mut().entity_mut(start_button_entity).insert(Interaction::Pressed);

        app.update();
        app.update(); // State transition happens in next frame

        // Check state
        let state = app.world().resource::<State<AppState>>().get();
        assert_eq!(state, &AppState::InGame, "State should be InGame after pressing Start");
    }

    #[test]
    fn test_exit_button_closes_app() {
        let mut app = App::new();
        app.add_plugins(StatesPlugin)
            .init_state::<AppState>()
            .add_message::<AppExit>()
            .add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
            .add_systems(Update, menu_action);

        app.update();

        // Find Exit button
        let mut exit_button_entity = None;
        let mut query = app.world_mut().query::<(&Text, &ChildOf)>();
        
        for (text, parent) in query.iter(app.world()) {
            if text.0 == "Exit" {
                exit_button_entity = Some(parent.parent());
                break;
            }
        }
        
        let exit_button_entity = exit_button_entity.expect("Exit button not found");

        // Simulate press
        app.world_mut().entity_mut(exit_button_entity).insert(Interaction::Pressed);

        app.update();

        // Check for AppExit event
        let events = app.world().resource::<Messages<AppExit>>();
        assert!(!events.is_empty(), "AppExit event should be sent after pressing Exit");
    }
}
