use crate::{AssetServer, Color, Commands, Component, Fixed, HALF_SCREEN_SIZE, IsGameEnded, PosState, Query, Res, ResMut, Resource, Rng, RngResource, Score, ScoreDisplay, ShowAabbGizmo, Sprite, Text, Time, Timer, Transform, With, format};

#[derive(Component, PartialEq)]
pub struct Pillar;

#[derive(Resource)]
pub struct PillarTimer(pub Timer);

#[derive(Resource)]
pub struct PillarConfig {
    pub velocity: f32,
    pub span: f32,
}

pub fn increment_pillar_timer(time: Res<Time<Fixed>>, mut timer: ResMut<PillarTimer>) {
    timer.0.tick(time.delta());
}

#[must_use] pub fn pillar_timer(timer: Res<PillarTimer>) -> bool {
    timer.0.finished()
}

pub fn spawn_pillars(
    mut commands: Commands,
    pillar_config: Res<PillarConfig>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut rng: ResMut<RngResource>,
    is_game_ended: Res<IsGameEnded>,
    mut query: Query<&mut Text, With<ScoreDisplay>>,
) {
    let random_height = rng
        .rng
        .random_range(-HALF_SCREEN_SIZE + pillar_config.span..540.0 - pillar_config.span);
    // update score
    if !is_game_ended.0 {
        score.0 += 1;
    }
    query.single_mut().expect("No score found!").0 = format!("Score: {}", score.0);
    for i in [-1.0, 1.0] {
        commands.spawn((
            Sprite::from_image(asset_server.load("dof.png")),
            Transform::from_xyz(0.0, random_height + pillar_config.span * i, 0.0),
            PosState {
                pos: 960.0,
                velocity: 0.0,
            },
            Pillar,
            ShowAabbGizmo {
                color: Some(Color::default()),
            },
        ));
    }
}

pub fn pillar_movement(
    time: Res<Time<Fixed>>,
    pillar_config: Res<PillarConfig>,
    mut query: Query<&mut PosState, With<Pillar>>,
) {
    for mut pillar_state in &mut query {
        pillar_state.pos -= pillar_config.velocity * time.delta_secs();
    }
}
