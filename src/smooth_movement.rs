use crate::{pillar::Pillar, Fixed, Player, PosState, Query, Res, Time, Transform};
use bevy::math::VectorSpace;

pub fn smooth_movement(
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
