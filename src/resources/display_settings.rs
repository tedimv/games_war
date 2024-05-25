use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct DisplaySettings {
    pub resolution: ResolutionOpt,
}



#[derive(Debug, Clone)]

pub struct ResolutionOpt {
    pub width: i16,
    pub height: i16,
}
