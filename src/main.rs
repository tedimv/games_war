use bevy::prelude::*;
use bevy::{
    input::keyboard::KeyboardInput,
    prelude::*,
    window::{Cursor, CursorGrabMode, WindowMode},
};
use bevy_asset_loader::prelude::*;
use bevy_eventlistener::prelude::*;
use std::process::Command;

mod plugins;

#[derive(AssetCollection, Resource)]
struct SpriteAssets {
    #[asset(path = "backgrounds/1.png")]
    background_1: Handle<Image>,
    #[asset(path = "backgrounds/2.png")]
    background_2: Handle<Image>,

    #[asset(path = "ui/buttons/play.png")]
    btn_play: Handle<Image>,
    #[asset(path = "ui/buttons/play_active.png")]
    btn_play_active: Handle<Image>,

    #[asset(path = "ui/buttons/quit.png")]
    btn_quit: Handle<Image>,
    #[asset(path = "ui/buttons/quit_active.png")]
    btn_quit_active: Handle<Image>,

    // background: Option<Handle<Image>>,
    #[asset(path = "backgrounds/in_menu/1.png")]
    background: Handle<Image>,
    #[asset(path = "cards/back.png")]
    card_back: Handle<Image>,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum StateGame {
    MainMenu,
    Settings,
    InGame,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum StateGameSpeed {
    Playing,
    PausedSettings,
    PausedTime,
}

#[derive(Resource, Debug)]
struct DisplaySettings {
    resolution: ResolutionOpt,
}

#[derive(Component)]
struct Card {
    strength: u8,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_background(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands
        .spawn((
            Card { strength: 2 },
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|root| {
            root.spawn(
                (SpriteSheetBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1920., 1280.)),
                        ..Default::default()
                    },
                    texture: sprite_assets.background.clone(),
                    transform: Transform {
                        scale: Vec3::new(1., 1., 1.),
                        translation: Vec3::new(-950., -530., 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            );
        });
}

fn switch_states(
    // input: KeyboardInput,
    keys_pressed: ResMut<'_, ButtonInput<KeyCode>>,
    state: Res<State<StateGame>>,
    mut next_state: ResMut<NextState<StateGame>>,
) {
    for key in keys_pressed.get_just_pressed().into_iter() {
        if *key == KeyCode::Enter {
            match state.get() {
                StateGame::MainMenu => {
                    println!("switching from MainMenu to InGame");
                    next_state.set(StateGame::InGame);
                }
                StateGame::InGame => {
                    println!("switching from InGame to MainMenu");
                    next_state.set(StateGame::MainMenu);
                }
                StateGame::Settings => {
                    println!("switching from Settings to MainMenu");
                    next_state.set(StateGame::Settings);
                }
            };
        }
    }
}

fn listen_to_space(
    keys_pressed: ResMut<'_, ButtonInput<KeyCode>>,
    // dispatch_event: EventWriter<KeyboardInput>,
) {
    let pressed_space = keys_pressed
        .get_just_pressed()
        .find(|key| *(*key) == KeyCode::Space);

    if None != pressed_space {
        println!(
            "---------------- SPAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACE -------------------------------"
        );
    }
}

#[derive(Debug, Clone)]

struct ResolutionOpt {
    pub width: i16,
    pub height: i16,
}

#[derive(Component)]
struct Button;

fn spawn_menu_buttons(mut commands: Commands, assets: Res<SpriteAssets>) {
    let btn_node = ButtonBundle {
        style: Style {
            height: Val::Px(120.),
            width: Val::Px(300.),
            ..Default::default()
        },
        ..Default::default()
    };

    fn print_hello() {
        println!("Heelo world ONCLICK");
    }

    commands
        .spawn(btn_node.clone())
        .with_children(|root| {
            root.spawn((SpriteBundle {
                texture: assets.btn_play.clone(),
                transform: Transform {
                    translation: Vec3::new(50., 20., 1.),
                    ..Default::default()
                },
                ..Default::default()
            }));
        });

    commands.spawn(btn_node.clone()).with_children(|root| {
        root.spawn(SpriteBundle {
            texture: assets.btn_quit.clone(),
            transform: Transform {
                translation: Vec3::new(50., 220., 1.),
                ..Default::default()
            },
            ..Default::default()
        });
    });
}

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                present_mode: bevy::window::PresentMode::AutoNoVsync,
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
        }))
        .init_collection::<SpriteAssets>()
        .insert_state(StateGame::MainMenu)
        .insert_resource(display_init)
        .add_systems(Startup, (spawn_camera, spawn_background))
        .add_systems(Update, switch_states)
        .add_systems(
            Update,
            (listen_to_space, spawn_menu_buttons).run_if(in_state(StateGame::MainMenu)),
        )
        .run();
}
