mod states;
use crate::states::AppState;
use bevy::camera::Camera2d;
use bevy::prelude::*; // Added back explicit import

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
        .run();
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
}
