#![no_std]
use bevy::prelude::*;
use world::Position;

#[derive(Component)]
pub struct Player;

pub struct WithPlayer;
impl Plugin for WithPlayer {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("spawn_player", SystemStage::single(spawn_player));
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn_empty()
        .insert(Player)
        .insert(Position { x: 0, y: 0 });
}
