use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle
};

// Give a z value to the ball so it stays on top
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BreakoutPlugin))
        .run();
}

pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, apply_velocity)
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Create the Ball
    commands.spawn(
        (MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED))
    );
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

/*
fn generate_blocks(width: u32) -> Vec<MaterialMesh2dBundle> {
    let num_blocks = 10;

    for  

}
*/

