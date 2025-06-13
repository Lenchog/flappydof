use crate::{pillar::Pillar, Assets, HALF_SCREEN_SIZE, Image, IsGameEnded, Player, PlayerSprite, Query, Res, ResMut, Transform, With};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};

pub fn collision(
    player_query: Query<&Transform, With<Player>>,
    pillar_query: Query<&Transform, With<Pillar>>,
    sprites: Res<Assets<Image>>,
    image_handle: Res<PlayerSprite>,
    mut is_game_ended: ResMut<IsGameEnded>,
) {
    let image_dimensions = sprites
        .get(&image_handle.0)
        .expect("No sprite found!")
        .size();
    let player_transform = &player_query.single();
    let player_collision = Aabb2d::new(
        player_transform
            .as_ref()
            .expect("No player transform found!")
            .translation
            .truncate(),
        image_dimensions.as_vec2(),
    );
    for pillar_transform in &pillar_query {
        let pillar_collision = Aabb2d::new(
            pillar_transform.translation.truncate(),
            pillar_transform.scale.truncate(),
        );
        let is_colliding = player_collision.intersects(&pillar_collision);
        let is_oob = player_transform
            .as_ref()
            .expect("No player transform found!")
            .translation
            .y
            < -HALF_SCREEN_SIZE;
        if is_colliding || is_oob {
            is_game_ended.0 = true;
        }
    }
}
