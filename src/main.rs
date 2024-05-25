mod components;
mod plugins;
mod resources;

use bevy::prelude::*;
use bevy::window::{Cursor, CursorGrabMode, WindowMode};
use bevy_asset_loader::prelude::*;
use components::game_state::GameState;
use plugins::background::BackgroundPlugin;
use plugins::camera::CameraPlugin;
use plugins::navigation::NavigationPlugin;
use plugins::routes::main_menu::MainMenuPlugin;
use resources::display_settings::{DisplaySettings, ResolutionOpt};
use resources::sprite_assets::SpriteAssets;

fn main() {
    let resolution_legacy = ResolutionOpt {
        width: 800,
        height: 600,
    };
    let resolution_full_hd = ResolutionOpt {
        width: 1920,
        height: 1280,
    };

    let resolution_options = vec![resolution_legacy.clone(), resolution_full_hd.clone()];
    let display_init = DisplaySettings {
        resolution: resolution_options
            .get(resolution_options.len() - 1)
            .unwrap()
            .clone(),
    };

    App::new()
        .add_plugins((
            CameraPlugin,
            BackgroundPlugin,
            MainMenuPlugin,
            NavigationPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    transparent: true,
                    decorations: false,
                    cursor: Cursor {
                        grab_mode: CursorGrabMode::Confined,
                        hit_test: true,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }),
        ))
        .init_collection::<SpriteAssets>()
        .insert_state(GameState::MainMenu)
        .insert_resource(display_init)
        .run();
}
