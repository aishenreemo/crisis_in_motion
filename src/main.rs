use bevy::prelude::*;
use bevy::window::WindowMode;
use grid::InfiniteGrid;
use grid::InfiniteGridPlugin;
use palette::ColorPalette;

mod grid;
mod palette;

fn main() {
    let window_title = "Crisis in Motion".to_owned();

    #[cfg(debug_assertions)]
    let window_title = format!("{} (bevy_dev)", window_title);

    let window = Window {
        title: window_title,
        mode: WindowMode::Windowed,
        ..default()
    };

    let window_plugin = WindowPlugin {
        primary_window: Some(window),
        ..default()
    };

    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(window_plugin));
    app.add_plugins(InfiniteGridPlugin);
    app.insert_resource(ClearColor(ColorPalette::KIZU.bg));
    app.insert_resource(ColorPalette::default());
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn(InfiniteGrid::default());
}
