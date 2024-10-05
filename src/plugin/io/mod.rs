mod event;
pub mod resource;
mod system;

use bevy::prelude::*;
use event::Input;
use resource::Terminal;
use system::*;

pub struct IoPlugin;
impl Plugin for IoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Terminal::init())
            .add_event::<Input>()
            .add_systems(Update, read_events)
            .observe(on_input);
    }
}
