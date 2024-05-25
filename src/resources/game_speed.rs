use bevy::ecs::schedule::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateGameSpeed {
    Playing,
    PausedSettings,
    PausedTime,
}
