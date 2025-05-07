use bevy::{
    math::{
        VectorSpace,
        bounding::{Aabb2d, IntersectsVolume},
    },
    prelude::*,
    render::camera::ScalingMode,
};
use rand::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(IsGameEnded(false))
        .insert_resource(GravityConfig(6000.0))
        .insert_resource(PillarDistance(300.0))
        .insert_resource(MovementConfig {
            max_speed: 2000.0,
            min_speed: 1500.0,
        })
        .insert_resource(PillarSpawnConfig {
            timer: Timer::new(Duration::from_millis(2000), TimerMode::Repeating),
        })
        .insert_resource(PillarVelocity(1000.0))
        .insert_resource(RngResource {
            rng: SmallRng::from_os_rng(),
        })
        .add_systems(
            FixedUpdate,
            (
                player_movement,
                spawn_pillars,
                pillar_movement,
                check_collision,
            ),
        )
        .add_systems(Update, (jump.run_if(run_if_not_ended), smooth_movement))
        .run();
}

#[derive(Resource)]
struct MovementConfig {
    max_speed: f32,
    min_speed: f32,
}

#[derive(Resource)]
struct PillarDistance(f32);

#[derive(Resource)]
struct PlayerSprite(Handle<Image>);

#[derive(Resource)]
struct RngResource {
    rng: SmallRng,
}

#[derive(Resource)]
struct GravityConfig(f32);

#[derive(Component)]
struct PosState {
    pos: f32,
    velocity: f32,
}

#[derive(Resource)]
struct PillarVelocity(f32);

#[derive(Resource)]
struct IsGameEnded(bool);

#[derive(Component, PartialEq)]
struct Player;

#[derive(Component, PartialEq)]
struct Pillar;

#[derive(Resource)]
struct PillarSpawnConfig {
    timer: Timer,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1080.0,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
    let velocity = 0.0;
    // spawn the player
    commands.spawn((
        Sprite::from_image(asset_server.load("dof.png")),
        PosState {
            velocity,
            pos: 540.0,
        },
        Player,
        ShowAabbGizmo {
            color: Some(Color::default()),
        },
    ));
    commands.insert_resource(PlayerSprite(asset_server.load("dof.png")));
}

fn spawn_pillars(
    mut commands: Commands,
    mut pillar: ResMut<PillarSpawnConfig>,
    pillar_distance: Res<PillarDistance>,
    asset_server: Res<AssetServer>,
    time: Res<Time<Fixed>>,
    mut rng: ResMut<RngResource>,
) {
    pillar.timer.tick(time.delta());
    let random_height = rng
        .rng
        .random_range(-540.0 + pillar_distance.0..540.0 - pillar_distance.0);
    if pillar.timer.finished() {
        commands.spawn((
            Sprite::from_image(asset_server.load("dof.png")),
            Transform::from_xyz(0.0, random_height + pillar_distance.0, 0.0),
            PosState {
                pos: 960.0,
                velocity: 0.0,
            },
            Pillar,
            ShowAabbGizmo {
                color: Some(Color::default()),
            },
        ));
        commands.spawn((
            Sprite::from_image(asset_server.load("dof.png")),
            Transform::from_xyz(0.0, random_height - pillar_distance.0, 0.0),
            PosState {
                pos: 960.0,
                velocity: 0.0,
            },
            Pillar,
            ShowAabbGizmo {
                color: Some(Color::default()),
            },
        ));
    };
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn player_movement(
    time: Res<Time<Fixed>>,
    gravity: Res<GravityConfig>,
    mut sprite_position: Query<&mut PosState, With<Player>>,
) {
    for mut player_state in &mut sprite_position {
        player_state.velocity -= gravity.0 * time.delta_secs();
        player_state.pos += player_state.velocity * time.delta_secs();
    }
}

fn pillar_movement(
    time: Res<Time<Fixed>>,
    velocity: Res<PillarVelocity>,
    mut query: Query<&mut PosState, With<Pillar>>,
) {
    for mut pillar_state in &mut query {
        pillar_state.pos -= velocity.0 * time.delta_secs();
    }
}

fn smooth_movement(
    time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &mut PosState,
        Option<&Player>,
        Option<&Pillar>,
    )>,
) {
    for (mut transform, state, player, pillar) in &mut query {
        let a = time.overstep_fraction();
        let future_position = state.pos + state.velocity * time.delta_secs();
        if player.is_some() {
            transform.translation.y = VectorSpace::lerp(state.pos, future_position, a);
        };
        if pillar.is_some() {
            transform.translation.x = VectorSpace::lerp(state.pos, future_position, a);
        }
    }
}

fn jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    movement: Res<MovementConfig>,
    mut player_state: Query<&mut PosState, With<Player>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut player_state in &mut player_state {
            if player_state.velocity < 0.0 {
                player_state.velocity = movement.min_speed
            } else if player_state.velocity > movement.max_speed {
                player_state.velocity = movement.max_speed
            } else {
                player_state.velocity += movement.min_speed
            }
        }
    }
}

fn check_collision(
    player_query: Query<&Transform, With<Player>>,
    pillar_query: Query<&Transform, With<Pillar>>,
    sprites: Res<Assets<Image>>,
    image_handle: Res<PlayerSprite>,
    mut is_game_ended: ResMut<IsGameEnded>,
) {
    let image_dimensions = sprites.get(&image_handle.0).unwrap().size();
    let player_transform = player_query.single();
    let player_collision = Aabb2d::new(
        player_transform.translation.truncate(),
        image_dimensions.as_vec2(),
    );
    for pillar_transform in &pillar_query {
        let pillar_collision = Aabb2d::new(
            pillar_transform.translation.truncate(),
            pillar_transform.scale.truncate(),
        );
        if player_collision.intersects(&pillar_collision) {
            is_game_ended.0 = true;
        }
    }
}

fn run_if_not_ended(is_game_ended: Res<IsGameEnded>) -> bool {
    !is_game_ended.0
}
