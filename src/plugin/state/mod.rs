mod context;
use bevy::prelude::*;
use context::{Context, ContextPlugin};

pub struct StatesPlugin;
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::state::app::StatesPlugin)
            .add_plugins(ContextPlugin)
            .insert_state(Context::Initial);
    }
}
