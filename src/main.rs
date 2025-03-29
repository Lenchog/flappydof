use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(GravityConfig(6000.0))
        .insert_resource(MovementConfig {
            max_speed: 2000.0,
            min_speed: 1500.0,
        })
        .add_systems(FixedUpdate, player_movement)
        .add_systems(Update, (jump, smooth_movement))
        .run();
}

#[derive(Resource)]
struct MovementConfig {
    max_speed: f32,
    min_speed: f32,
}

#[derive(Resource)]
struct GravityConfig(f32);

#[derive(Component)]
struct PlayerState {
    y_pos: f32,
    velocity: f32,
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
        Transform::from_xyz(0., 0., 0.),
        PlayerState {
            velocity,
            y_pos: 540.0,
        },
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn player_movement(
    time: Res<Time<Fixed>>,
    gravity: Res<GravityConfig>,
    mut sprite_position: Query<&mut PlayerState>,
) {
    for mut player_state in &mut sprite_position {
        player_state.velocity -= gravity.0 * time.delta_secs();
        player_state.y_pos += player_state.velocity * time.delta_secs();
    }
}

fn smooth_movement(
    time: Res<Time<Fixed>>,
    mut sprite_position: Query<(&mut Transform, &mut PlayerState)>,
) {
    for (mut transform, state) in &mut sprite_position {
        let a = time.overstep_fraction();
        let future_position = state.y_pos + state.velocity * time.delta_secs();
        transform.translation.y = state.y_pos.lerp(future_position, a);
    }
}

fn jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    movement: Res<MovementConfig>,
    mut player_state: Query<&mut PlayerState>,
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
