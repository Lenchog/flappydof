use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .add_systems(FixedUpdate, player_movement)
        .add_systems(Update, (jump, smooth_movement))
        .run();
}

const MAX_SPEED: f32 = 3000.0;
const MIN_SPEED: f32 = 1500.0;
const DEFAULT_JUMP: f32 = 1500.0;
const GRAVITY: f32 = 4000.0;

#[derive(Component)]
struct PlayerState {
    y_pos: f32,
    velocity: f32,
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    let velocity = 0.0;
    commands.spawn((
        Sprite::from_image(asset_server.load("dof.png")),
        Transform::from_xyz(0., 0., 0.),
        PlayerState { velocity, y_pos: 0.0 },
        Player,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn player_movement(
    time: Res<Time<Fixed>>,
    mut sprite_position: Query<&mut PlayerState, With<Player>>,
) {
    for mut player_state in &mut sprite_position {
        player_state.velocity -= GRAVITY * time.delta_secs();
        player_state.y_pos += player_state.velocity * time.delta_secs();
    }
}

fn smooth_movement(
    time: Res<Time<Fixed>>,
    mut sprite_position: Query<(&mut Transform, &mut PlayerState), With<Player>>,
) {
    for (mut transform, player_state) in &mut sprite_position {
        let a = time.overstep_fraction();
        //let a = time.delta_secs();
        let future_position = player_state.y_pos
            + player_state.velocity * time.delta_secs();
        transform.translation.y = player_state.y_pos.lerp(future_position, a);
        //transform.translation.y = player_state.y_pos;
    }
}

fn jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_state: Query<&mut PlayerState, With<Player>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for mut player_state in &mut player_state {
            match player_state.velocity {
                ..0. => player_state.velocity = MIN_SPEED,
                MAX_SPEED.. => player_state.velocity = MAX_SPEED,
                _ => player_state.velocity += DEFAULT_JUMP
            }
        };
    }
}
