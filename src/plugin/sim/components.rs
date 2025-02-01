use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Symbol(pub char);

#[derive(Component, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Material {
    Cloth,
    Iron,
}
