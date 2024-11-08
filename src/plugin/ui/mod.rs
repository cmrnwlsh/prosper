mod log;
mod splash;

use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use ratatui::widgets::Block;

pub fn ui(app: &mut App) {
    app.add_plugins(ContextGroup)
        .insert_state(Context::default());
}

pub struct ContextGroup;
impl PluginGroup for ContextGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(splash::context)
            .add(log::context)
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum Context {
    #[default]
    Initial,
    Log,
}

pub fn title_block(diagnostics: Res<DiagnosticsStore>) -> Block<'_> {
    Block::bordered().title(format!(
        " TPS: {:.2} ",
        diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
            .unwrap_or(f64::NAN)
    ))
}
