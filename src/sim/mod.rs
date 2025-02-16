pub mod components;

use bevy::prelude::*;

pub struct SimPlugin;
impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {});
    }
}
