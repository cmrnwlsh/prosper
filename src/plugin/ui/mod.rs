mod initial;
mod log;

use bevy::{app::PluginGroupBuilder, prelude::*};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Context {
    #[default]
    Initial,
    Log,
}

pub fn ui(app: &mut App) {
    app.add_plugins(ContextPlugins)
        .insert_state(Context::default());
}

pub struct ContextPlugins;
impl PluginGroup for ContextPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(log::context)
    }
}
