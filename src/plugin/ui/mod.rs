mod initial;
mod log;
mod splash;

use super::io::Input;
use bevy::{app::PluginGroupBuilder, prelude::*};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn plugin(app: &mut App) {
    app.add_plugins(ContextGroup)
        .init_state::<Context>()
        .insert_resource(ContextStack(vec![]))
        .add_event::<ForwardTransition>()
        .add_systems(Update, (listen_log, listen_exit, listen_back))
        .add_systems(PostUpdate, process_transition);
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

#[derive(Event, Debug)]
pub enum ForwardTransition {
    Context { current: Context, target: Context },
    Exit,
}

#[derive(Resource)]
struct ContextStack(Vec<Context>);

impl From<(Context, Context)> for ForwardTransition {
    fn from(value: (Context, Context)) -> Self {
        Self::Context {
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

fn listen_exit(mut events: EventReader<Input>, mut exit: EventWriter<ForwardTransition>) {
    events.read().for_each(|ev| {
        if let KeyEvent {
            code: KeyCode::Char('c'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::CONTROL,
            ..
        } = ev.0
        {
            exit.send(ForwardTransition::Exit);
        }
    })
}

fn process_transition(
    mut events: EventReader<ForwardTransition>,
    mut state: ResMut<NextState<Context>>,
    mut stack: ResMut<ContextStack>,
    mut exit: EventWriter<AppExit>,
) {
    events.read().for_each(|ev| match ev {
        ForwardTransition::Context { current, target } if target != current => {
            stack.0.push(*current);
            state.set(*target);
        }
        ForwardTransition::Exit => {
            exit.send(AppExit::Success);
        }
        _ => {}
    })
}

fn listen_back(
    mut events: EventReader<Input>,
    mut stack: ResMut<ContextStack>,
    mut state: ResMut<NextState<Context>>,
) {
    events.read().for_each(|ev| {
        if let KeyCode::Esc = ev.0.code {
            if let Some(c) = stack.0.pop() {
                state.set(c)
            }
        }
    })
}
