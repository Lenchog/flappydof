use bevy::prelude::*;
use rand::prelude::*;

pub mod setup;

pub mod pillar;

pub mod player_movement;

pub mod smooth_movement;

pub mod collision;

pub const HALF_SCREEN_SIZE: f32 = 1080.0 / 2.0;

#[derive(Component)]
pub struct PosState {
    pub pos: f32,
    pub velocity: f32,
}

#[derive(Component, PartialEq)]
pub struct Player;

#[derive(Component, PartialEq)]
pub struct ScoreDisplay;

#[derive(Resource)]
pub struct MovementConfig {
    pub max_speed: f32,
    pub min_speed: f32,
    pub gravity: f32,
}

#[derive(Resource)]
pub struct Score(pub u16);

#[derive(Resource)]
pub struct PlayerSprite(pub Handle<Image>);

#[derive(Resource)]
pub struct RngResource {
    pub rng: SmallRng,
}

#[derive(Resource)]
pub struct IsGameEnded(pub bool);
