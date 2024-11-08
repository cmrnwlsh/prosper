mod data;
mod io;
mod log;
mod sim;
mod ui;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin},
    asset::AssetPlugin,
    diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    hierarchy::HierarchyPlugin,
    state::app::StatesPlugin,
    MinimalPlugins,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use data::data;
use io::io;
use log::log;
use std::time::Duration;
use ui::ui;

pub struct ProsperPlugins;
impl PluginGroup for ProsperPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EmbeddedAssetPlugin::default())
            .add_group(
                MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1. / 60.,
                ))),
            )
            .add(StatesPlugin)
            .add(AssetPlugin::default())
            .add(DiagnosticsPlugin)
            .add(FrameTimeDiagnosticsPlugin)
            .add(HierarchyPlugin)
            .add(io)
            .add(ui)
            .add(data)
            .add(log)
    }
}
