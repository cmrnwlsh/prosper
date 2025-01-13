pub mod components;
use bevy::prelude::*;

const MAP: &str = r#"
########################################
#......................................#
#......................................#
#......................................#
#......................................#
#......................................#
#......................................#
#......................................#
#......................................#
#......................................#
#......................................#
#......................................#
#......................................#
#......................................#
########################################
"#;

pub enum Tile {
    Wall,
    Floor,
    Empty,
}

#[derive(Resource)]
pub struct TileMap(pub Vec<Vec<Tile>>);

impl TileMap {
    pub fn symbol_grid(&self) -> Vec<Vec<char>> {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        Tile::Floor => '.',
                        Tile::Wall => '#',
                        Tile::Empty => ' ',
                    })
                    .collect()
            })
            .collect()
    }
}

impl Default for TileMap {
    fn default() -> Self {
        Self(
            MAP.split("\n")
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Tile::Wall,
                            '.' => Tile::Floor,
                            _ => Tile::Empty,
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(TileMap::default());
}
