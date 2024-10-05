mod context;
use bevy::prelude::*;

pub struct StatesPlugin;
impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.add_systems(Update, context::main::system);
        app.insert_state(Context::Main);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Context {
    Main,
}
