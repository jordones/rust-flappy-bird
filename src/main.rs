pub mod bird;
pub mod pipe;

use bevy::{prelude::*, render::camera::ScalingMode};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".into(),
                        resizable: false,
                        resolution: (240.0, 480.0).into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_systems(Startup, setup_camera)
        .add_plugins((bird::BirdPlugin, pipe::PipePlugin))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin { min_width: 60.0, min_height: 120.0 };
    commands.spawn(camera);
}