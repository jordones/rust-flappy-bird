pub mod bird;

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
                        resolution: (480.0, 960.0).into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_systems(Startup, setup_camera)
        .add_plugins(bird::BirdPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin { min_width: 120.0, min_height: 240.0 };
    commands.spawn(camera);
}