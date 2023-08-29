use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin}, sprite::{MaterialMesh2dBundle, collide_aabb::{collide, Collision}}, app::AppExit,
};
use rand::{random, Rng};

const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const BOTTOM_WALL: f32 = -300.;
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;

const PADDLE_SPEED: f32 = 700.0;
const WALL_THICKNESS: f32 = 10.0;
    // time that occurs within a physics step (1 second)
const TIME_STEP: f32 = 1.0 / 60.0;
  // 

// for ball 
const BALL_COLOR: Color = Color::rgb(0.4, 1.0, 0.1);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const BALL_SPEED: f32 = 300.0;
  //
const TOP_WALL: f32 = 300.0;
const WALL_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
//const BRICK_SIZE: Vec2 = Vec2::new(10., 3.);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;
const BRICK_COLOR: Color = Color::rgb(1.0, 0.1, 0.0);
const PADDLE_PADDING: f32 = 10.0;
const SPAWN_TIME: f32 = 50.0; 
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        //turbofish syntax
        .init_resource::<Scoreboard>()
        .init_resource::<BrickTimer>()
        .add_startup_system(infotext_system)
        .add_event::<CollisionEvent>()
         /* 
        .add_systems(
            (
                check_for_collisions,
                apply_velocity.before(check_for_collisions),
                move_paddle
                    .before(check_for_collisions)
                    .after(apply_velocity),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_system(update_score)
        .add_system(spawn_brick_timer)
        .add_system(bricks_over_time)
        .add_system(end_game)
        .add_system(bevy::window::close_on_esc)
       */
        .add_system(move_paddle)
        .add_system(apply_velocity)

        .add_system(check_for_collisions)
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_system(update_score)
        .add_system(spawn_brick_timer)
        .add_system(bricks_over_time)
        .add_system(end_game)
        .add_system(bevy::window::close_on_esc)
        .run();
}

impl Default for Scoreboard {
    fn default() -> Scoreboard {
        Scoreboard { score: 0 }
    }
}

#[derive(Resource)]
struct BrickTimer {
    pub timer: Timer,
}

impl Default for BrickTimer {
    fn default() -> Self {
        BrickTimer { timer: Timer::from_seconds(SPAWN_TIME, TimerMode::Repeating)
            }
    }
}

#[derive(Component)]
struct TextChanges;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Brick; 

#[derive(Component)]
struct Ball;
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

#[derive(Default)]
struct CollisionEvent;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct EndEvent;

#[derive(Component)]
struct Tile;

fn infotext_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut the_material: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(Camera2dBundle::default());
    
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                //scale: Vec3::new(3.0, 0.5, 0.5),
                ..default()
            },
            texture: asset_server.load("images/01-Breakout-Tiles.png"),
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
        //Tile,
    ));

    commands.spawn((        
         MaterialMesh2dBundle {
             mesh: meshes.add(shape::Circle::default().into()).into(),
             //material: the_material.add(ColorMaterial::from(BALL_COLOR)),
             material: the_material.add(ColorMaterial { 
                color: BALL_COLOR, 
                texture: Some(asset_server.load("images/58-Breakout-Tiles.png")),
            }),
             transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
             ..default()
         },
         Ball,
         Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
     ));    
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "HighScore: 1000",
                TextStyle { 
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
                    font_size: 40.0, 
                    color: TEXT_COLOR, 
                },
            ),
            TextSection::from_style(TextStyle { 
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0, 
                color: TEXT_COLOR, 
            }),  
        ])
        .with_style(Style { 
            position_type: PositionType::Absolute,
            position: UiRect {
                //left: Val::Px(5.0),
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                //top: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        
    );
    commands.spawn(TextBundle::from_section(
            "Type Esc to exit the game",
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::rgb(0.8, 0.2, 0.7),
            },
        )
        //.with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            },
            max_size: Size {
                width: Val::Px(300.),
                height: Val::Undefined,
            },
            ..default()
        })
    );
 
     // Add the walls to bound the ball
     commands.spawn(WallBundle::new(WallLocation::Left));
     commands.spawn(WallBundle::new(WallLocation::Right));
     commands.spawn(WallBundle::new(WallLocation::Top));
     commands.spawn(WallBundle::new(WallLocation::Bottom));

     assert!(BRICK_SIZE.x > 0.0);
    assert!(BRICK_SIZE.y > 0.0);

    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the number of columns,
    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
        // Space taken up by the bricks and gaps
        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;
    
    for row in 0..n_rows {
        for column in 0..n_columns {
            //let col = (n_columns + column) / 2; 
            let brick_position = Vec2::new(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        custom_size: Some(Vec2::new(1.0,1.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                        ..default()
                    },
                    texture: asset_server.load("images/07-Breakout-Tiles.png"),
                    ..default()
                },
                Brick,
                Collider,
            ));
        }
    }

}

enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}


impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
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

fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    // paddle 
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

        // Calculate the new horizontal paddle position based on player input
    let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;

        // Update the paddle position,
        // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut scoreboard: ResMut<Scoreboard>,
    //collider_query: Query<(Entity, &Transform), With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, transform, maybe_brick) in &collider_query {
        //for (collider_entity, transform) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            
            // Bricks should be despawned and increment the scoreboard on collision
            if maybe_brick.is_some() {
                scoreboard.score += 1;
                commands.entity(collider_entity).despawn();
                print!("Your score is {}\n", scoreboard.score);
            }
            
            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                //Collision::Bottom => {},
                Collision::Inside => { /* do nothing */ }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }

        }
    }
}

fn update_score(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<ScoreText>>
) {
    //let text = query.get_single_mut();
    //let mut text = query.single_mut();
    //text.sections[1].value = scoreboard.score.to_string();
    //the_text[1].value = scoreboard.score.to_string();
    //print!("Your score is {}", scoreboard.score);
    for mut text in &mut query {
        text.sections[1].value = scoreboard.score.to_string();
       
    }
}

fn end_game(
    mut end_screen: ResMut<Events<bevy::app::AppExit>>,
    scoreboard: ResMut<Scoreboard>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // end game at certain score 
    if scoreboard.score == 50 {
        //end_screen.send(AppExit)    
        println!("Game Over! Final Score: {}\n Better luck next time;) ", scoreboard.score);
        end_screen.send(AppExit);
    }
    if keyboard_input.pressed(KeyCode::Escape) {
        print!("Game ended early!\n Your score is {}\n", scoreboard.score);
    }
}

fn spawn_brick_timer(
    mut spawn_timer: ResMut<BrickTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
}

fn bricks_over_time (
    mut commands: Commands,
    spawn_timer: ResMut<BrickTimer>,
    asset_server: Res<AssetServer>,
) { 
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;
    let mut rng = rand::thread_rng();
    let random_row = rng.gen_range(0..5);
    let random_col = rng.gen_range(0..5);
    assert!(BRICK_SIZE.x > 0.0);
    assert!(BRICK_SIZE.y > 0.0);

    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the number of columns,
    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
        // Space taken up by the bricks and gaps
        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;
    
    //timer hits 0, set at repeating, so will start again. 
    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;
    let brick_position = Vec2::new(
        offset_x + 2 as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
        offset_y + 2 as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
    );
    if spawn_timer.timer.finished() {
        for row in 0..random_row {
            for column in 0..random_col {
                //let col = (n_columns + column) / 2; 
                let brick_position = Vec2::new(
                    offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                    offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
                );
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: BRICK_COLOR,
                            custom_size: Some(Vec2::new(1.0,1.0)),
                            ..default()
                        },
                        transform: Transform {
                            translation: brick_position.extend(0.0),
                            scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                            ..default()
                        },
                        texture: asset_server.load("images/07-Breakout-Tiles.png"),
                        ..default()
                    },
                    Brick,
                    Collider,
                ));
            }
        }

    }
}