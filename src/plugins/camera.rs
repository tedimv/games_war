use bevy::app::{Plugin, Startup};
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::prelude::Camera2dBundle;

#[derive(Clone, Debug)]
pub struct CameraPlugin;

#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            ..Default::default()
        },
        MyCameraMarker,
    ));
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_camera);
    }
}
