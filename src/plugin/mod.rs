mod io;
mod log;
mod state;

use std::time::Duration;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin},
    diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    hierarchy::HierarchyPlugin,
    MinimalPlugins,
};
use io::IoPlugin;
use log::LogPlugin;
use state::StatesPlugin;

pub struct TuiPlugins;
impl PluginGroup for TuiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(
                MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_micros(16670))),
            )
            .add(StatesPlugin)
            .add(LogPlugin)
            .add(DiagnosticsPlugin)
            .add(FrameTimeDiagnosticsPlugin)
            .add(HierarchyPlugin)
            .add(IoPlugin)
    }
}
