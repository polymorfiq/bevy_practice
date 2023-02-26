#![no_std]

use agents::Player;
use bevy::prelude::*;
use world::Position;

pub struct WithControls;
impl Plugin for WithControls {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(bevy::time::FixedTimestep::step(1.0 / 3.0))
                .with_system(move_player),
        );
    }
}

const PLAYER_SPEED: isize = 1;
fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    let mut move_x = 0;
    let mut move_y = 0;
    if keyboard_input.pressed(KeyCode::A) {
        move_x -= PLAYER_SPEED;
    }

    if keyboard_input.pressed(KeyCode::D) {
        move_x += PLAYER_SPEED;
    }

    if keyboard_input.pressed(KeyCode::W) {
        move_y += PLAYER_SPEED;
    }

    if keyboard_input.pressed(KeyCode::S) {
        move_y -= PLAYER_SPEED;
    }

    for mut pos in &mut query {
        pos.x += move_x;
        pos.y += move_y;
    }
}
