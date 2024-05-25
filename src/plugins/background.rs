use bevy::prelude::*;

use crate::{components::game_state::GameState, resources::sprite_assets::SpriteAssets};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_background_default)
            .add_systems(OnEnter(GameState::InGame), spawn_background_faded);
    }
}

fn spawn_background_default(
    mut commands: Commands,
    assets: Res<SpriteAssets>,
    backgrounds: Query<Entity, With<BackgroundComponent>>,
) {
    let prev_bg = backgrounds.get_single();
    if let Ok(bg) = prev_bg {
        commands.entity(bg).despawn_recursive();
    }

    commands.spawn((
        SpriteBundle {
            texture: assets.background_1_in_menu.clone(),
            ..Default::default()
        },
        BackgroundComponent,
    ));
}

fn spawn_background_faded(
    mut commands: Commands,
    assets: Res<SpriteAssets>,
    backgrounds: Query<Entity, With<BackgroundComponent>>,
) {
    let prev_bg = backgrounds.get_single();
    if let Ok(bg) = prev_bg {
        commands.entity(bg).despawn_recursive();
    }

    commands.spawn((
        SpriteBundle {
            texture: assets.background_1_in_game.clone(),
            ..Default::default()
        },
        BackgroundComponent,
    ));
}

#[derive(Component)]
struct BackgroundComponent;
