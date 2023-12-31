
use bevy::{prelude::*, window::WindowResolution};
use rand::{self, Rng};

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpawnTimer>()
            .add_systems(Startup, spawn_pipe)
            .add_systems(Update, (scroll_pipes, advance_timer, spawn_pipes, despawn_pipes));
    }
}

//// Timer impl

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

impl SpawnTimer {
    fn new() -> Self {
        SpawnTimer {
            timer: Timer::from_seconds(1.5, TimerMode::Repeating)
        }
    }
}

impl Default for SpawnTimer {
    fn default() -> Self {
        SpawnTimer::new()
    }
}

fn advance_timer(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>
) {
    spawn_timer.timer.tick(time.delta());
}

//// End timer impl

#[derive(Component, PartialEq)]
enum ScreenPosition {
    Top,
    Bottom
}

#[derive(Component)]
enum PipeSize {
    Sm,
    Md,
    Lg
}


#[derive(Component)]
struct PipeParent {
    pipes: Vec<PipeSet>,
    timer: Timer,
}

#[derive(Component)]
struct Pipe;

#[derive(Component)]
struct PipeSet(PipeBundle, PipeBundle);

impl PipeSet {
    fn new(asset_server: &Res<AssetServer>, window_resolution: &WindowResolution) -> Self {
        PipeSet {
            0: PipeBundle::new(ScreenPosition::Bottom, get_rand_pipe_size(), asset_server, window_resolution),
            1: PipeBundle::new(ScreenPosition::Top, get_rand_pipe_size(), asset_server, window_resolution),
        }
    }
}

fn get_rand_pipe_size() -> PipeSize {
    let rand_val = rand::thread_rng().gen_range(0, 3);
    rand_to_pipe_size(rand_val)
}

fn rand_to_pipe_size(rand_int: i32) -> PipeSize {
    if rand_int == 0 {
        PipeSize::Sm
    }
    else if rand_int == 1 {
        PipeSize::Md
    }
    else {
        PipeSize::Lg
    }
}

#[derive(Bundle)]
struct PipeBundle {
    marker: Pipe,
    screen_pos: ScreenPosition,
    sprite: SpriteBundle,
}

impl PipeBundle {
    fn new(
        screen_pos: ScreenPosition,
        size: PipeSize,
        asset_server: &Res<AssetServer>,
        window_resolution: &WindowResolution
    ) -> Self {
        let texture = asset_server.load("pipe.png");

        let offset = match size {
            PipeSize::Sm => 0.0,
            PipeSize::Md => 8.0,
            PipeSize::Lg => 16.0,
        };

        let flip_y = screen_pos != ScreenPosition::Bottom;
        let x_pos = window_resolution.width() / 8.0 + 8.0;
        let y_pos = if flip_y {
            // /8 to match resolution and /8 to fit co-ordinate system
            // -8 is half the texture height (TBD how to get this from the Handle<Image>)
            window_resolution.height() / 8.0 - offset
        } else {
            -window_resolution.height() / 8.0 + offset
        };

        PipeBundle { 
            marker: Pipe, 
            screen_pos, 
            sprite: SpriteBundle { 
                texture,
                sprite: Sprite {
                    flip_y,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: x_pos,
                        y: y_pos,
                        z: 0.1,
                    },
                    ..Default::default()
                },
                ..Default::default()
            } 
        }
    }
}

fn spawn_pipe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Window>
) {
    let window = query.single();

    // Temp: Spawning in a test PipeSet
    let x = PipeSet::new(&asset_server, &window.resolution);

    commands.spawn(x.0);
    commands.spawn(x.1);
}

fn scroll_pipes(
    mut query: Query<&mut Transform, With<Pipe>>,
    time: Res<Time>
) {
    for mut pipe_transform in query.iter_mut() {
        pipe_transform.translation.x -= time.delta_seconds() * 16.0;
    }
}

fn spawn_pipes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Window>,
    spawn_timer: ResMut<SpawnTimer>

) {
    if spawn_timer.timer.finished() {
        info!("finished timer");
        let window = query.single();

        // Temp: Spawning in a test PipeSet
        let x = PipeSet::new(&asset_server, &window.resolution);
    
        commands.spawn(x.0);
        commands.spawn(x.1);
    }
}

fn despawn_pipes(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Pipe>>,
    window_query: Query<&Window>
) {
    let window_width = -window_query.single().width() / 8.0 - 8.0;
    for (entity, transform) in query.iter() {
        if transform.translation.x < window_width {
            commands.entity(entity).despawn();
        }
    }
}