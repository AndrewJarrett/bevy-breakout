use bevy::prelude::*;

use super::{despawn_screen, GameState};

const GLOW_INCREMENT: f32 = 1.0;
const GLOW_MIN_COLOR: f32 = 0.0;
const GLOW_MAX_COLOR: f32 = 2.0;
const GLOW_DEGREES_WRAP: f32 = 180.0;
const SPLASH_COLOR: Color = Color::rgba(GLOW_MIN_COLOR, GLOW_MIN_COLOR, GLOW_MIN_COLOR, 1.0);
const SPLASH_TIME_SECONDS: f32 = 5.0;

// This plugin will display a splash screen with Bevy logo for 
// 3 seconds before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, (countdown, glow).run_if(in_state(GameState::Splash)))
            //.add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Tag for Splash icon
#[derive(Component)]
struct SplashIcon;

#[derive(Component)]
struct Glow {
    degrees: f32,
}

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("splash.png");

    // Display the logo
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..default()
            },
            texture: icon,
            sprite: Sprite {
                color: SPLASH_COLOR,
                custom_size: Some(Vec2::splat(200.0)),
                ..default()
            },
            ..default()
        },
        SplashIcon,
        OnSplashScreen
    ));

    // Create a counter for the glow
    commands.spawn(Glow { degrees: 0.0 });

    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(SPLASH_TIME_SECONDS, TimerMode::Once)));
}

// Make the icon "glow"
fn glow(
    mut icon_query: Query<&mut Sprite, With<SplashIcon>>,
    mut glow_query: Query<&mut Glow>,
    time: Res<Time>
) {
    let mut sprite = icon_query.single_mut();
    let mut glow = glow_query.single_mut();

    glow.degrees += GLOW_INCREMENT * time.delta_seconds();
    glow.degrees = glow.degrees % GLOW_DEGREES_WRAP;
    info!("glow.degrees: {}", glow.degrees);

    let new_color: f32 = GLOW_MAX_COLOR * f32::sin(glow.degrees);

    sprite.color = Color::rgba(new_color, new_color, new_color, 1.0);
    info!("new_color: {}", new_color);
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
