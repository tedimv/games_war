use crate::{
    components::{game_state::GameState, nav_link::NavLink},
    resources::sprite_assets::SpriteAssets,
};
use bevy::prelude::*;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_menu_buttons)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu_buttons);
    }
}

fn spawn_menu_buttons(mut commands: Commands, assets: Res<SpriteAssets>) {
    let column = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(100.),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(200., 50., 1.),
            ..Default::default()
        },
        ..Default::default()
    };

    let buttons = vec![
        (UiImage::new(assets.btn_play.clone()), GameState::InGame),
        (UiImage::new(assets.btn_quit.clone()), GameState::Settings),
        (UiImage::new(assets.btn_quit.clone()), GameState::Quit),
    ];

    commands
        .spawn((column, MainMenuControl {}))
        .with_children(|root| {
            buttons.into_iter().for_each(|(image_handle, game_state)| {
                let link = NavLink(game_state);
                let btn = ButtonBundle {
                    image: image_handle,
                    style: Style {
                        height: Val::Px(120.),
                        width: Val::Px(300.),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                root.spawn((btn, link));
            });
        });
}

fn despawn_menu_buttons(mut commands: Commands, controls: Query<Entity, With<MainMenuControl>>) {
    controls.iter().for_each(|control_ui| {
        commands.entity(control_ui).despawn_recursive();
    });
}
#[derive(Component)]
struct MainMenuControl;
