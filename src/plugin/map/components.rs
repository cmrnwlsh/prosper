use crate::plugin::sim::components::Position;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct TileBundle {
    pub marker: Tile,
    pub position: Position,
    pub kind: TileKind,
}

impl Default for TileBundle {
    fn default() -> Self {
        Self {
            marker: Tile,
            position: Position { x: 0, y: 0 },
            kind: TileKind::Empty,
        }
    }
}

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub enum TileKind {
    Wall,
    Floor,
    Empty,
}
