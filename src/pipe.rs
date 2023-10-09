use bevy::{prelude::*, window::WindowResolution};

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_pipe);
    }
}

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
        // TODO: Randomize sm/md/lg
        PipeSet {
            0: PipeBundle::new(ScreenPosition::Bottom, PipeSize::Sm, asset_server, window_resolution),
            1: PipeBundle::new(ScreenPosition::Top, PipeSize::Md, asset_server, window_resolution),
        }
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
        let x_pos = window_resolution.width() / 8.0 - 8.0;
        let y_pos = if flip_y {
            // /2 to match resolution and /2 to fit co-ordinate system
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