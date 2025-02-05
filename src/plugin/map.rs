use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use super::sim::components::Position;

static GRID_SIZE: usize = 1000;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileMap>();
    }
}

#[derive(Default)]
pub struct Tile {
    feature_set: HashSet<Feature>,
}

#[derive(Default)]
pub enum Feature {
    #[default]
    Plain,
    Mountain,
    River,
    Ocean,
    Lake,
    Road,
}

#[derive(Resource)]
pub struct TileMap(HashMap<Position, Tile>);

impl Default for TileMap {
    fn default() -> Self {
        Self(
            (0..GRID_SIZE)
                .flat_map(|x| (0..GRID_SIZE).map(move |y| (Position { x, y }, default())))
                .collect(),
        )
    }
}
