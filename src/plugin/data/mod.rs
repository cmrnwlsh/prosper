pub mod components;
mod items;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoaderGroup).init_state::<LoadState>();
    }
}

struct LoaderGroup;
impl PluginGroup for LoaderGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(items::apparel::loader)
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LoadState {
    #[default]
    Loading,
    Loaded,
}
