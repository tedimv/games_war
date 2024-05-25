use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct NavLink(pub GameState);
