use bevy::prelude::*;

use crate::plugin::sim::components::Position;

#[derive(Bundle)]
pub struct TileBundle {
    pub marker: Tile,
    pub position: Position,
}

#[derive(Component)]
pub struct Tile;
