use bevy::prelude::*;

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
    fn new(asset_server: &Res<AssetServer>) -> Self {
        // TODO: Randomize sm/md/lg
        PipeSet {
            0: PipeBundle::new(ScreenPosition::Bottom, PipeSize::Md, asset_server),
            1: PipeBundle::new(ScreenPosition::Top, PipeSize::Sm, asset_server),
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
        asset_server: &Res<AssetServer>
    ) -> Self {
        let texture = match size {
            PipeSize::Sm => asset_server.load("pipe_sm.png"),
            PipeSize::Md => asset_server.load("pipe_md.png"),
            PipeSize::Lg => asset_server.load("pipe_lg.png"),
        };

        let flip_y = screen_pos != ScreenPosition::Bottom;
        // TODO: Set position based on screen_pos
        PipeBundle { 
            marker: Pipe, 
            screen_pos, 
            sprite: SpriteBundle { 
                texture,
                sprite: Sprite {
                    flip_y,
                    ..Default::default()
                },
                ..Default::default() 
            } 
        }
    }
}

fn spawn_pipe(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Temp: Spawning in a test PipeSet
    let x = PipeSet::new(&asset_server);

    commands.spawn(x.0);
    commands.spawn(x.1);
}