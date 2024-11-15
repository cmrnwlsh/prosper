mod initial;
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

pub fn plugin(app: &mut App) {
    app.add_plugins(ContextGroup)
        .init_state::<Context>()
        .add_systems(Update, (listen_log, listen_exit, listen_back));
}

struct ContextGroup;
impl PluginGroup for ContextGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(splash::context)
            .add(initial::context)
            .add(log::context)
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Context {
    #[default]
    Splash,
    Initial,
    Log,
}

#[derive(Resource)]
struct ContextStack(Vec<Context>);

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

fn listen_back(
    mut events: EventReader<Input>,
    mut stack: ResMut<ContextStack>,
    mut state: ResMut<NextState<Context>>,
) {
    events.read().for_each(|ev| {
        if let (KeyCode::Esc, Some(c)) = (ev.0.code, stack.0.pop()) {
            state.set(c)
        }
    })
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
