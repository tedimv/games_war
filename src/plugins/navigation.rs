use bevy::prelude::*;

use crate::components::game_state::GameState;

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, navigate);
    }
}

fn navigate(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
    key_input: ResMut<'_, ButtonInput<KeyCode>>,
) {
    for k in key_input.get_just_pressed() {
        if *k == KeyCode::Enter {
            match *current_state.get() {
                GameState::MainMenu => {
                    println!("=> GameState::InGame");
                    next_state.set(GameState::InGame);
                }
                GameState::InGame => {
                    println!("=> GameState::MainMenu");
                    next_state.set(GameState::MainMenu);
                }
                _ => {}
            }
        }
    }
}
