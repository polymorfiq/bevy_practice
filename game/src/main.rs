#![feature(stmt_expr_attributes)]

use bevy::{log::LogPlugin, prelude::*, window::PresentMode};
use ui::GameState;

fn main() {
    let mut app = App::new();
    let mut plugins = DefaultPlugins.build();

    plugins = plugins.set(LogPlugin {
        level: bevy::log::Level::WARN,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    });

    plugins = plugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "My Game Window".to_string(),
            transparent: true,
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::Windowed,
            ..default()
        },
        ..default()
    });

    #[cfg(debug_assertions)]
    plugins = plugins.set(LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "warn,wgpu_core=warn,wgpu_hal=warn,game=debug,environment=debug".into(),
    });

    app.add_plugins(plugins)
        .add_state(GameState::InGame)
        .add_plugin(controls::WithControls)
        .add_plugin(environment::WithEnvironment)
        .add_plugin(agents::WithPlayer)
        .insert_resource(ClearColor(Color::rgba(0.0, 0.0, 0.0, 0.5)));

    app.run();
}
