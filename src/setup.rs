use crate::{AssetServer, Camera2d, Color, Commands, HALF_SCREEN_SIZE, OrthographicProjection, Player, PlayerSprite, PosState, Projection, Res, ScoreDisplay, ShowAabbGizmo, SpawnRelated, Sprite, Text};
use bevy::render::camera::ScalingMode;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1080.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.spawn((Text::new("Score: 0"), ScoreDisplay));
    let velocity = 0.0;
    // spawn the player
    commands.spawn((
        Sprite::from_image(asset_server.load("dof.png")),
        PosState {
            velocity,
            pos: HALF_SCREEN_SIZE,
        },
        Player,
        ShowAabbGizmo {
            color: Some(Color::default()),
        },
    ));
    commands.insert_resource(PlayerSprite(asset_server.load("dof.png")));
}
