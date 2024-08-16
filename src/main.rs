mod plugin;
mod resource;

use bevy::{
    app::ScheduleRunnerPlugin,
    diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use plugin::io::TuiPlugin;
use resource::terminal::Terminal;
use std::{
    panic::{set_hook, take_hook},
    time::Duration,
};

fn main() {
    let hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        Terminal::restore();
        hook(panic_info);
    }));
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_micros(16670))),
            DiagnosticsPlugin,
            FrameTimeDiagnosticsPlugin,
            HierarchyPlugin,
            TuiPlugin,
        ))
        .run();
}
