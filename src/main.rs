mod plugin;

use bevy::prelude::*;
use plugin::ProsperPlugins;

fn main() {
    App::new().add_plugins(ProsperPlugins).run();
}
