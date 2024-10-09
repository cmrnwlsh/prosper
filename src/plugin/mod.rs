mod data;
mod io;
mod log;
mod ui;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin},
    asset::AssetPlugin,
    diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    hierarchy::HierarchyPlugin,
    MinimalPlugins,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use data::DataPlugin;
use io::IoPlugin;
use log::LogPlugin;
use std::time::Duration;
use ui::UiPlugin;

pub struct TuiPlugins;
impl PluginGroup for TuiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EmbeddedAssetPlugin::default())
            .add_group(
                MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_micros(16670))),
            )
            .add(AssetPlugin::default())
            .add(IoPlugin)
            .add(UiPlugin)
            .add(DiagnosticsPlugin)
            .add(FrameTimeDiagnosticsPlugin)
            .add(HierarchyPlugin)
            .add(LogPlugin)
            .add(DataPlugin)
    }
}
