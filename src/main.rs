mod data;
mod io;
mod log;
mod sim;
mod ui;

use bevy::{
    app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin},
    diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    hierarchy::HierarchyPlugin,
    prelude::*,
    state::app::StatesPlugin,
    MinimalPlugins,
};
use data::DataPlugin;
use io::IoPlugin;
use log::LogPlugin;
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
            .add(SimPlugin)
            .add(DataPlugin)
            .add(LogPlugin)
    }
}

fn main() {
    App::new().add_plugins(ProsperPlugins).run();
}
