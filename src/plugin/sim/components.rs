use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Clone, Copy)]
pub struct Symbol(pub char);
