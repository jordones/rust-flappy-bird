use bevy::prelude::*;

#[derive(Component)]
pub struct Bird();

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird);
    }
}

fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("bird.png");

    commands.spawn(SpriteBundle {
        texture,
        ..default()
    });
}