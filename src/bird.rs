use bevy::prelude::*;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_bird)
            .add_systems(Update, apply_gravity_to_bird);
    }
}

/// ## Bird
/// Primary player component
#[derive(Component)]
struct Bird;

#[derive(Bundle)]
struct BirdBundle {
    marker: Bird,
    sprite: SpriteBundle,
}

/// Spawns a bird entity
/// 
/// Should be run once during scene initialization
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

/// Applies a continuous force to the Bird entity
/// 
/// It is assumed there will only be one instance of a Bird in the world
fn apply_gravity_to_bird(
    mut query: Query<(&mut Transform, With<Bird>)>,
    time: Res<Time>
) {
    if let Ok((mut bird_transform, _)) = query.get_single_mut() {
        bird_transform.translation.y -= time.delta_seconds() * 60.0;
    }
}
