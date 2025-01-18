mod data;
mod io;
mod log;
mod map;
mod sim;
mod ui;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin},
    diagnostic::DiagnosticsPlugin,
    hierarchy::HierarchyPlugin,
    state::app::StatesPlugin,
    MinimalPlugins,
};
use std::time::Duration;

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
            .add(HierarchyPlugin)
            .add(io::plugin)
            .add(data::plugin)
            .add(ui::plugin)
            .add(map::plugin)
            .add(sim::plugin)
            .add(log::plugin)
    }
}
