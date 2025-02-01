pub mod components;

use bevy::{prelude::*, utils::info};
use components::{Position, Symbol};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, |mut commands: Commands| {
        debug!("debug");
        commands.spawn((Symbol('@'), Position { x: 20, y: 7 }));
    });
}
