use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Event)]
pub enum GameState {
    MainMenu,
    Settings,
    InGame,
    Quit,
}
