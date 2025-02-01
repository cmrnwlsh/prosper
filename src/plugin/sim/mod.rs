use super::data::components::{Position, Symbol};
use bevy::prelude::*;

pub struct SimPlugin;
impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            info!("info");
            debug!("debug");
            commands.spawn((Symbol('@'), Position { x: 20, y: 7 }));
        });
    }
}
