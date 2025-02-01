mod log;
mod primary;
mod splash;

use super::io::Input;
use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

const TITLE_BAR: &str = " -PROSPER- ";

pub fn fps(diag: Res<DiagnosticsStore>) -> String {
    if let Some(fps) = diag
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        format!("{:.2}", fps)
    } else {
        "".into()
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ContextGroup)
            .init_state::<Context>()
            .insert_resource(ContextStack(vec![]))
            .add_event::<ForwardTransition>()
            .add_systems(Update, (listen_log, listen_exit, listen_back))
            .add_systems(PostUpdate, process_transition);
    }
}

struct ContextGroup;
impl PluginGroup for ContextGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(splash::context)
            .add(primary::context)
            .add(log::context)
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Context {
    #[default]
    Splash,
    Primary,
    Log,
}

#[derive(Event, Debug)]
pub struct ForwardTransition {
    current: Context,
    target: Context,
}

#[derive(Resource)]
struct ContextStack(Vec<Context>);

impl From<(Context, Context)> for ForwardTransition {
    fn from(value: (Context, Context)) -> Self {
        Self {
            current: value.0,
            target: value.1,
        }
    }
}

fn listen_log(
    mut events: EventReader<Input>,
    mut next: EventWriter<ForwardTransition>,
    ctx: Res<State<Context>>,
) {
    events.read().for_each(|ev| {
        if let KeyCode::Char('`') = ev.0.code {
            next.send((*ctx.get(), Context::Log).into());
        };
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

fn process_transition(
    mut events: EventReader<ForwardTransition>,
    mut state: ResMut<NextState<Context>>,
    mut stack: ResMut<ContextStack>,
) {
    events.read().for_each(|ev| match ev {
        ForwardTransition { current, target } if target != current => {
            stack.0.push(*current);
            state.set(*target);
        }
        _ => {}
    })
}

fn listen_back(
    mut events: EventReader<Input>,
    mut stack: ResMut<ContextStack>,
    mut state: ResMut<NextState<Context>>,
    mut exit: EventWriter<AppExit>,
) {
    events.read().for_each(|ev| {
        if let KeyCode::Esc | KeyCode::Backspace = ev.0.code {
            if let Some(c) = stack.0.pop() {
                state.set(c)
            } else {
                exit.send(AppExit::Success);
            }
        }
    })
}
