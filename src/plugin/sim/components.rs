use bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct Symbol(pub char);
