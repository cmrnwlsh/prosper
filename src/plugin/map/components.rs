use bevy::prelude::*;

#[derive(Bundle)]
pub struct TileBundle {
    pub marker: Tile,
}

#[derive(Component)]
pub struct Tile;
