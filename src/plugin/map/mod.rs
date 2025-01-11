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

enum Tile {
    Wall,
    Floor,
    Empty,
}

#[derive(Resource)]
struct TileMap(Vec<Vec<Tile>>);

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
pub fn build(app: &mut App) {
    app.insert_resource(TileMap::default())
        .add_systems(Startup, init);
}

fn init(mut commands: Commands) {}
