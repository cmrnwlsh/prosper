pub mod components;

use bevy::prelude::*;
use components::{Position, Symbol};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((Symbol('@'), Position { x: 20, y: 7 }));
    });
}
