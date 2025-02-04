mod io;
mod log;
mod map;
mod sim;
mod ui;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin},
    diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    hierarchy::HierarchyPlugin,
    state::app::StatesPlugin,
    MinimalPlugins,
};
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
            .add(StatesPlugin)
            .add(DiagnosticsPlugin)
            .add(FrameTimeDiagnosticsPlugin)
            .add(HierarchyPlugin)
            .add(IoPlugin)
            .add(UiPlugin)
            .add(MapPlugin)
            .add(SimPlugin)
            .add(LogPlugin)
    }
}
