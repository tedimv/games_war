use bevy::prelude::*;

use crate::resources::sprite_assets::SpriteAssets;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}

fn spawn_background(mut commands: Commands, assets: Res<SpriteAssets>) {
    commands.spawn(SpriteBundle {
        texture: assets.background.clone(),
        ..Default::default()
    });
}

