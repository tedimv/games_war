use bevy::app::{Plugin, Startup};
use bevy::ecs::system::Commands;
use bevy::prelude::Camera2dBundle;

#[derive(Clone, Debug)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}
