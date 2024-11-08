use super::{title_block, Context};
use crate::plugin::{
    io::{Input, Terminal},
    log::LogStore,
};
use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use ratatui::{
    crossterm::event::KeyCode,
    widgets::{Paragraph, Wrap},
    Frame,
};

pub fn context(app: &mut App) {
    app.insert_resource(ScrollState(0)).add_systems(
        Update,
        (render, listen_scroll).run_if(in_state(Context::Log)),
    );
}

#[derive(Resource)]
struct ScrollState(u16);

fn render(
    mut term: ResMut<Terminal>,
    diagnostics: Res<DiagnosticsStore>,
    logs: Res<LogStore>,
    scroll: Res<ScrollState>,
) {
    term.draw(|frame: &mut Frame| {
        frame.render_widget(
            Paragraph::new(logs.0.join("\n"))
                .scroll((scroll.0, 0))
                .wrap(Wrap { trim: true })
                .block(title_block(diagnostics)),
            frame.area(),
        )
    })
    .unwrap();
}

fn listen_scroll(mut events: EventReader<Input>, mut scroll: ResMut<ScrollState>) {
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
