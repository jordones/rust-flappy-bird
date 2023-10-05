use bevy::prelude::*;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_bird)
            .add_systems(Update, apply_gravity_to_bird);
    }
}

#[derive(Component)]
struct Bird;

#[derive(Bundle)]
struct BirdBundle {
    marker: Bird,
    sprite: SpriteBundle,
}

fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("bird.png");

    commands.spawn(BirdBundle {
        marker: Bird,
        sprite: SpriteBundle {
            texture,
            ..default()
        }
    });

}

fn apply_gravity_to_bird(
    mut query: Query<(&mut Transform, With<Bird>)>,
    time: Res<Time>
) {
    let mut bird_transform = query.single_mut();

    bird_transform.0.translation.y -= time.delta_seconds() * 60.0;
}
