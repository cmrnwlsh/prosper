use bevy::prelude::*;

#[derive(Component, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
#[derive(Component)]
pub struct Symbol(pub char);

#[derive(Component)]
pub struct Name<'a>(pub &'a str);

#[derive(Component)]
pub struct Settlement;

#[derive(Component)]
pub struct Player;
