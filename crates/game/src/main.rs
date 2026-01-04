mod states;
use crate::states::AppState;
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use bevy::text::TextPlugin;
use bevy::ui::{AlignItems, JustifyContent, PositionType, Val};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), main_menu_setup)
        .run();
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(Camera2dBundle::default());
    // commands.spawn(
    //     TextBundle::from_section(
    //         "Main Menu",
    //         TextStyle {
    //             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //             font_size: 100.0,
    //             color: Color::WHITE,
    //         },
    //     )
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         bottom: Val::Px(100.0),
    //         right: Val::Px(100.0),
    //         ..default()
    //     }),
    // );
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(Camera2dBundle::default());
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("branding/bevy_logo_dark_big.png"),
    //     ..default()
    // });
}

#[cfg(test)]
mod tests {
    use super::*;
    use states::AppState;

    #[test]
    fn test_app_has_app_state() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_state::<AppState>(); // Initialize the AppState
        app.update(); // Run one frame for systems to execute
        assert!(app.world().get_resource::<State<AppState>>().is_some());
    }

    #[test]
    fn test_main_menu_text_is_present() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), TextPlugin))
            .init_state::<AppState>()
            .add_systems(OnEnter(AppState::MainMenu), main_menu_setup);
        app.update(); // Run one frame for systems to execute

        // Check if the "Main Menu" text exists
        let mut query = app.world().query::<&Text>();
        let mut found_text = false;
        for text in query.iter(&app.world()) {
            if text.value == "Main Menu" {
                found_text = true;
                break;
            }
        }
        assert!(found_text, "Main Menu text not found.");
    }
}
