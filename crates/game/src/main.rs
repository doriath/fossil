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

fn main_menu_setup(mut _commands: Commands) {
    // UI elements will be added here
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

#[cfg(test)]
mod tests {
    use super::*;
    use states::AppState;

    #[test]
    fn test_app_has_app_state() {
        let mut app = App::new();
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.init_state::<AppState>(); // Initialize the AppState
        app.update(); // Run one frame for systems to execute
        assert!(app.world().get_resource::<State<AppState>>().is_some());
    }

    // #[test]
    // fn test_main_menu_text_is_present() {
    //     let mut app = App::new();
    //     app.add_plugins(bevy::state::app::StatesPlugin)
    //         .init_state::<AppState>()
    //         .add_systems(OnEnter(AppState::MainMenu), main_menu_setup);
    //     app.update(); // Run one frame for systems to execute

    //     // Check if the "Main Menu" text exists
    //     let mut query = app.world_mut().query::<&Text>();
    //     let mut found_text = false;
    //     for text in query.iter(&app.world()) {
    //         if text.0 == "Main Menu" {
    //             found_text = true;
    //             break;
    //         }
    //     }
    //     assert!(found_text, "Main Menu text not found.");
    // }

    // #[test]
    // fn test_main_menu_buttons_are_present() {
    //     let mut app = App::new();
    //     app.add_plugins(DefaultPlugins)
    //         .init_state::<AppState>()
    //         .add_systems(OnEnter(AppState::MainMenu), main_menu_setup);
    //     app.update();

    //     let button_query = app
    //         .world()
    //         .query_filtered::<(Entity, &Children), With<Button>>();
    //     let mut text_query = app.world().query::<&Text>();

    //     let mut start_button_found = false;
    //     let mut exit_button_found = false;

    //     for (_button_entity, children) in button_query.iter(&app.world()) {
    //         for child_entity in children.iter() {
    //             if let Ok(text) = text_query.get(&app.world(), child_entity) {
    //                 if !text.0.is_empty() && text.0 == "Start" {
    //                     start_button_found = true;
    //                 }
    //                 if !text.0.is_empty() && text.0 == "Exit" {
    //                     exit_button_found = true;
    //                 }
    //             }
    //         }
    //     }
    //     assert!(start_button_found, "Start button not found in Main Menu.");
    //     assert!(exit_button_found, "Exit button not found in Main Menu.");
    // }
}
