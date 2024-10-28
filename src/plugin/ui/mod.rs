mod initial;

use bevy::{app::PluginGroupBuilder, prelude::*, state::app::StatesPlugin};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Context {
    #[default]
    Initial,
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StatesPlugin, ContextPlugins))
            .insert_state(Context::default());
    }
}

pub struct ContextPlugins;
impl PluginGroup for ContextPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(initial::ContextPlugin)
    }
}
