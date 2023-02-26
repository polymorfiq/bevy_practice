#![no_std]

use agents::Player;
use bevy::prelude::*;
use ui::GameState;
use world::Position;

pub struct WithEnvironment;
impl Plugin for WithEnvironment {
    fn build(&self, app: &mut App) {
        app.add_startup_stage("setup_environment", SystemStage::single(setup_environment))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(bevy::time::FixedTimestep::step(1.0 / 60.0))
                    .with_system(track_player),
            );
    }
}

#[derive(Component)]
pub struct GridSquare;

const OFFSET_X: isize = -800;
const OFFSET_Y: isize = -600;
const SIZE_X: usize = 50;
const SIZE_Y: usize = 50;
const WIDTH: usize = 50;
const HEIGHT: usize = 50;
const GUTTER_X: usize = 5;
const GUTTER_Y: usize = 5;
const EMPTY_SQUARE_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 0.5);
const FILLED_SQUARE_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
fn setup_environment(mut commands: Commands) {
    for row in 0..SIZE_Y {
        for col in 0..SIZE_X {
            commands
                .spawn(SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: (OFFSET_X + ((WIDTH + GUTTER_X) * col) as isize) as f32,
                            y: (OFFSET_Y + ((HEIGHT + GUTTER_Y) * row) as isize) as f32,
                            z: 0.0,
                        },
                        ..default()
                    },
                    sprite: Sprite {
                        color: EMPTY_SQUARE_COLOR,
                        custom_size: Some(Vec2 {
                            x: SIZE_X as f32,
                            y: SIZE_Y as f32,
                        }),
                        ..default()
                    },
                    ..default()
                })
                .insert(Position {
                    x: col as isize,
                    y: row as isize,
                })
                .insert(GridSquare);
        }
    }
}

fn track_player(
    players: Query<&Position, With<Player>>,
    mut squares: Query<(&Position, &mut Sprite), With<GridSquare>>,
) {
    for (square_pos, mut square_sprite) in &mut squares {
        for player_pos in &players {
            if square_pos.x == player_pos.x && square_pos.y == player_pos.y {
                square_sprite.color = FILLED_SQUARE_COLOR;
            } else {
                square_sprite.color = EMPTY_SQUARE_COLOR;
            }
        }
    }
}
