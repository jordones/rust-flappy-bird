use bevy::prelude::*;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_bird)
            .add_systems(Update, (apply_gravity_to_bird, handle_input, handle_velocity));
    }
}

/// ## Bird
/// Primary player component
#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Bundle)]
struct BirdBundle {
    marker: Bird,
    velocity: Velocity,
    sprite: SpriteBundle,
}

/// Spawns a bird entity
/// 
/// Should be run once during scene initialization
fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("bird.png");

    commands.spawn(BirdBundle {
        marker: Bird,
        velocity: Velocity(Vec2 { x: 0.0, y: 0.0 }),
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
    mut query: Query<&mut Transform, With<Bird>>,
    time: Res<Time>
) {
    if let Ok(mut bird_transform) = query.get_single_mut() {
        bird_transform.translation.y -= time.delta_seconds() * 80.0;
    }
}

fn handle_input(
    mut query: Query<&mut Velocity, With<Bird>>,
    input: Res<Input<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.0.y += 80.0;
    }
}

fn handle_velocity(
    mut query: Query<(&mut Transform, &mut Velocity), With<Bird>>,
    time: Res<Time>
) {
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        if velocity.0.y <= 0.0 {
            return;
        }

        transform.translation.y += velocity.0.y * time.delta_seconds();
        velocity.0.y -= velocity.0.y * time.delta_seconds();
    }
}