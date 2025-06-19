use crate::{
    ButtonInput, Fixed, IsGameEnded, KeyCode, MovementConfig, Player, PosState, Query, Res, Time,
    With,
};

pub fn player_movement(
    time: Res<Time<Fixed>>,
    movement_config: Res<MovementConfig>,
    mut sprite_position: Query<&mut PosState, With<Player>>,
) {
    for mut player_state in &mut sprite_position {
        player_state.velocity -= movement_config.gravity * time.delta_secs();
        player_state.pos += player_state.velocity * time.delta_secs();
    }
}

pub fn jump(movement: Res<MovementConfig>, mut player_state: Query<&mut PosState, With<Player>>) {
    let mut player_state = player_state.single_mut().expect("No player found!");
    if player_state.velocity < 0.0 {
        player_state.velocity = movement.min_speed;
    } else if player_state.velocity > movement.max_speed {
        player_state.velocity = movement.max_speed;
    } else {
        player_state.velocity += movement.min_speed;
    }
}

#[must_use]
pub fn check_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    is_game_ended: Res<IsGameEnded>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space) && !is_game_ended.0
}
