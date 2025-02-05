pub mod components;

use bevy::prelude::*;
use components::{Position, Symbol};

pub struct SimPlugin;
impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Symbol('@'), Position { x: 7, y: 20 }));
        });
    }
}
