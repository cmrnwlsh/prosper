use crate::plugin::{
    io::{Input, Terminal},
    log::LogStore,
};
use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use ratatui::{
    crossterm::event::KeyCode,
    widgets::{Block, Paragraph},
    Frame,
};

use super::{fps, Context, TITLE_BAR};

pub fn context(app: &mut App) {
    app.insert_resource(State(0)).add_systems(
        Update,
        (render, listen_scroll).run_if(in_state(Context::Log)),
    );
}

#[derive(Resource)]
struct State(u16);

fn render(
    mut term: ResMut<Terminal>,
    logs: Res<LogStore>,
    scroll: Res<State>,
    diag: Res<DiagnosticsStore>,
) {
    term.draw(|frame: &mut Frame| {
        frame.render_widget(
            Paragraph::new(logs.0.join("\n"))
                .scroll((scroll.0, 0))
                .block(Block::bordered().title(format!("{TITLE_BAR}{} ", fps(diag)))),
            frame.area(),
        )
    })
    .unwrap();
}

fn listen_scroll(mut events: EventReader<Input>, mut scroll: ResMut<State>) {
    events.read().for_each(|ev| {
        let s = &mut scroll.0;
        *s = match ev.0.code {
            KeyCode::Up => s.saturating_sub(1),
            KeyCode::Down => s.saturating_add(1),
            KeyCode::PageUp => s.saturating_sub(10),
            KeyCode::PageDown => s.saturating_add(10),
            _ => *s,
        }
    })
}
