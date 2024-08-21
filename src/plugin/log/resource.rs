use bevy::prelude::*;

#[derive(Resource)]
pub struct LogStore(pub Vec<String>);
