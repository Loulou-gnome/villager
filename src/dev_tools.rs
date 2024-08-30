//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::input::common_conditions::input_toggle_active;
use bevy::{
    dev_tools::{
        states::log_transitions,
        ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_plugins(DebugUiPlugin);
    // Add plugins to show info about the world.>>
    app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, TOGGLE_KEY)));
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
