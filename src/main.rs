mod plugin;

use bevy::prelude::*;
use plugin::TuiPlugins;

fn main() {
    App::new().add_plugins(TuiPlugins).run();
}
