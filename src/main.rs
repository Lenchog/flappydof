use bevy::{prelude::*, window::PresentMode};
use flappydof::{collision::collision, pillar::{increment_pillar_timer, pillar_movement, pillar_timer, spawn_pillars}, player_movement::{check_jump, jump, player_movement}, setup, smooth_movement::smooth_movement, IsGameEnded, MovementConfig, RngResource, Score, pillar};
use rand::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_systems(Startup, setup::setup)
        .insert_resource(IsGameEnded(false))
        .insert_resource(Score(0))
        .insert_resource(pillar::PillarConfig {
            velocity: 1000.0,
            span: 300.0,
        })
        .insert_resource(MovementConfig {
            max_speed: 2000.0,
            min_speed: 1500.0,
            gravity: 6000.0,
        })
        .insert_resource(pillar::PillarTimer(Timer::new(
            Duration::from_millis(2000),
            TimerMode::Repeating,
        )))
        .insert_resource(RngResource {
            rng: SmallRng::from_os_rng(),
        })
        .add_systems(
            FixedUpdate,
            (
                // physics
                player_movement,
                pillar_movement,
                collision,
                // pillar spawning
                increment_pillar_timer,
                spawn_pillars.run_if(pillar_timer),
            ),
        )
        .add_systems(Update, (jump.run_if(check_jump), smooth_movement))
        .run();
}
