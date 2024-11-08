mod log;
mod splash;

use super::io::Input;
use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    widgets::Block,
};

pub fn ui(app: &mut App) {
    app.add_plugins(ContextGroup)
        .init_state::<Context>()
        .add_systems(Update, (listen_log, listen_exit));
}

struct ContextGroup;
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
    Splash,
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

fn listen_log(mut events: EventReader<Input>, mut state: ResMut<NextState<Context>>) {
    events.read().for_each(|ev| {
        if let KeyCode::Char('`') = ev.0.code {
            state.set(Context::Log);
        }
    })
}

fn listen_exit(mut events: EventReader<Input>, mut exit: EventWriter<AppExit>) {
    events.read().for_each(|ev| {
        if let KeyEvent {
            code: KeyCode::Char('c'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::CONTROL,
            ..
        } = ev.0
        {
            exit.send(AppExit::Success);
        }
    })
}
