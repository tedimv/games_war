use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "backgrounds/in_menu/1.png")]
    pub background_1_in_menu: Handle<Image>,
    #[asset(path = "backgrounds/in_game/1.png")]
    pub background_1_in_game: Handle<Image>,

    #[asset(path = "backgrounds/in_menu/2.png")]
    pub background_2_in_menu: Handle<Image>,
    #[asset(path = "backgrounds/in_game/2.png")]
    pub background_2_in_game: Handle<Image>,

    #[asset(path = "ui/buttons/play.png")]
    pub btn_play: Handle<Image>,
    #[asset(path = "ui/buttons/play_active.png")]
    pub btn_play_active: Handle<Image>,

    #[asset(path = "ui/buttons/quit.png")]
    pub btn_quit: Handle<Image>,
    #[asset(path = "ui/buttons/quit_active.png")]
    pub btn_quit_active: Handle<Image>,

    // background: Option<Handle<Image>>,
    #[asset(path = "cards/back.png")]
    pub card_back: Handle<Image>,
}
