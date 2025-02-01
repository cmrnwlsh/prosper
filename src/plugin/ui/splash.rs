use super::{fps, Context, ForwardTransition, TITLE_BAR};
use crate::plugin::{
    data::LoadState,
    io::{Input, Terminal},
    log::{LogEvent, LogStore},
};
use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Layout},
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

const SPLASH: &str = r#"
░▒▓███████▓▒░░▒▓███████▓▒░ ░▒▓██████▓▒░ ░▒▓███████▓▒░▒▓███████▓▒░░▒▓████████▓▒░▒▓███████▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░
░▒▓███████▓▒░░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓███████▓▒░░▒▓██████▓▒░ ░▒▓███████▓▒░ 
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓███████▓▒░░▒▓█▓▒░      ░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░
"#;

pub fn context(app: &mut App) {
    app.add_systems(
        Update,
        (
            render.run_if(in_state(Context::Splash)),
            listen_input
                .run_if(in_state(Context::Splash))
                .run_if(in_state(LoadState::Loaded)),
        ),
    );
}

fn render(
    mut term: ResMut<Terminal>,
    diag: Res<DiagnosticsStore>,
    load_state: Res<State<LoadState>>,
    logs: Res<LogStore>,
) {
    term.draw(|frame: &mut Frame| {
        frame.render_widget(
            Paragraph::new(format!(
                "{}\n{}",
                SPLASH,
                if *load_state.get() == LoadState::Loaded {
                    "press spacebar to continue".into()
                } else {
                    logs.0
                        .iter()
                        .filter_map(|log| {
                            if let LogEvent::Error(s) = log {
                                Some(s.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                }
            ))
            .wrap(Wrap { trim: false })
            .centered(),
            Layout::vertical([Constraint::Ratio(1, 3); 3]).split(frame.area())[1],
        );
        frame.render_widget(
            Block::bordered().title(format!("{TITLE_BAR}{} ", fps(diag))),
            frame.area(),
        );
    })
    .unwrap();
}

fn listen_input(mut events: EventReader<Input>, mut next: EventWriter<ForwardTransition>) {
    events.read().for_each(|ev| {
        if let KeyCode::Char(' ') = ev.0.code {
            next.send((Context::Splash, Context::Primary).into());
        }
    });
}
