use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle
};

// Paddle constants
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_SPEED: f32 = 500.0;
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_PADDING: f32 = 10.0; // How close paddle can get to the wall
const PADDLE_COLOR: Color = Color::LIME_GREEN;

// Give a z value to the ball so it stays on top
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const BALL_COLOR: Color = Color::CYAN;

// Set up wall constants
const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const TOP_WALL: f32 = 300.0;
const BOTTOM_WALL: f32 = -300.0;
const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::RED;

// Set up blocks
const BLOCK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
// Exact values
const GAP_BETWEEN_PADDLE_AND_BLOCKS: f32 = 270.0;
const GAP_BETWEEN_BLOCKS: f32 = 5.0;
// These are lower bounds that are used as we compute the # of blocks
const GAP_BETWEEN_BLOCKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BLOCKS_AND_SIDES: f32 = 20.0;
const BLOCK_COLOR: Color = Color::PINK;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BreakoutPlugin))
        .run();
}

pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (
                    apply_velocity,
                    move_paddle,
                    check_collisions,
                ).chain()
            )
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Block;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Bundle)]
struct WallBundle {
    // We need a sprite and a collider for each wall
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

// We will have four walls
enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    // Get the position of each wall
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    // Get the size of the area
    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        // Make sure we have a non-zero arena
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Top | WallLocation::Bottom => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert Vec2 to Vec3 to give it a z-ccoordinate
                    // which is used to determine the order of sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2d objects needs to be 1.0 or 
                    // the ordering will be affected
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }

}

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
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED))
    );

    // Create the Paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));

    // Create the walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Bottom));

    // Generate all the blocks
    generate_blocks(commands, paddle_y);
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

fn check_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Block>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // Check for a collision
    for (collider_entity, transform, maybe_block) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            // Send a collision event so other systems can react to it
            collision_events.send_default();

            // Blocks need to disappear when hit
            if maybe_block.is_some() {
                commands.entity(collider_entity).despawn();
            }

            // Reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // Only reflect if the Ball's velocity is going in the opposite direction
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => {}
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }

}

fn move_paddle(
    mut paddle_query: Query<&mut Transform, With<Paddle>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut paddle_transform = paddle_query.single_mut();
    let mut direction = 0.0;

    if keys.pressed(KeyCode::Left) { 
        direction -= 1.0;
    }

    if keys.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();

    // Make sure paddle stops before each wall
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

fn generate_blocks(mut commands: Commands, paddle_y: f32) {
    let total_width_of_blocks = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEEN_BLOCKS_AND_SIDES;
    let bottom_edge_of_blocks = paddle_y + GAP_BETWEEN_PADDLE_AND_BLOCKS;
    let total_height_of_blocks = TOP_WALL - bottom_edge_of_blocks - GAP_BETWEEN_BLOCKS_AND_CEILING;

    assert!(total_width_of_blocks > 0.0);
    assert!(total_height_of_blocks > 0.0);

    // Given our space available, compute number of rows and columns that will fit
    let n_columns = (total_width_of_blocks / (BLOCK_SIZE.x + GAP_BETWEEN_BLOCKS)).floor() as usize;
    let n_rows = (total_height_of_blocks / (BLOCK_SIZE.y + GAP_BETWEEN_BLOCKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the # of columns, the space on the top and 
    // sides of the blocks is only a lower bound, not an exact value
    let center_of_blocks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_blocks = center_of_blocks
        // Space taken up by the blocks
        - (n_columns as f32 / 2.0 * BLOCK_SIZE.x)
        // Space taken up by the gaps
        - (n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BLOCKS);

    let offset_x = left_edge_of_blocks + BLOCK_SIZE.x / 2.0;
    let offset_y = bottom_edge_of_blocks + BLOCK_SIZE.y / 2.0;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let block_position = Vec2::new(
                offset_x + column as f32 * (BLOCK_SIZE.x + GAP_BETWEEN_BLOCKS),
                offset_y + row as f32 * (BLOCK_SIZE.y + GAP_BETWEEN_BLOCKS)
            );

            // Create the block
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: BLOCK_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: block_position.extend(0.0),
                        scale: Vec3::new(BLOCK_SIZE.x, BLOCK_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Block,
                Collider
            ));
        }
    }
}

