mod splash;
mod menu;
mod breakout;

use bevy::{
    prelude::*,
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping
    },
    input::common_conditions::input_toggle_active,
};
use bevy_inspector_egui::quick::{
    WorldInspectorPlugin,
    ResourceInspectorPlugin
};

use crate::{
    breakout::{BreakoutPlugin, Scoreboard},
    splash::SplashPlugin,
    menu::MenuPlugin,
};

pub const TEXT_COLOR: Color = Color::WHITE;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(u32);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, SplashPlugin, MenuPlugin, BreakoutPlugin, 
            WorldInspectorPlugin::default().run_if(
                input_toggle_active(false, KeyCode::Grave)
            ),
            ResourceInspectorPlugin::<Scoreboard>::default().run_if(
                input_toggle_active(false, KeyCode::Grave)
            )
        ))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<GameState>()
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_systems(Startup, app_setup)
        .run();
}

fn app_setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // Required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // Use tonemapping that desaturates to white
            ..default()
        },
        // Enable bloom for the camera
        BloomSettings {
            intensity: 0.5,
            low_frequency_boost: 2.0,
            low_frequency_boost_curvature: 0.3,
            high_pass_frequency: 0.3,
            composite_mode: BloomCompositeMode::Additive,
            ..default()
        }
    ));
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

