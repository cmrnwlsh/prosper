mod data;
mod io;
mod log;
mod map;
mod sim;
mod ui;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin},
    asset::AssetPlugin,
    diagnostic::DiagnosticsPlugin,
    hierarchy::HierarchyPlugin,
    state::app::StatesPlugin,
    MinimalPlugins,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use data::DataPlugin;
use io::IoPlugin;
use log::LogPlugin;
use map::MapPlugin;
use sim::SimPlugin;
use std::time::Duration;
use ui::UiPlugin;

pub struct ProsperPlugins;
impl PluginGroup for ProsperPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(
                MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1. / 60.,
                ))),
            )
            .add(EmbeddedAssetPlugin::default())
            .add(StatesPlugin)
            .add(AssetPlugin::default())
            .add(DiagnosticsPlugin)
            .add(HierarchyPlugin)
            .add(IoPlugin)
            .add(UiPlugin)
            .add(DataPlugin)
            .add(MapPlugin)
            .add(SimPlugin)
            .add(LogPlugin)
    }
}
