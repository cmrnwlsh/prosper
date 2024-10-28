use crate::plugin::{
    io::{Input, Terminal},
    log::LogStore,
};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use ratatui::{
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

pub struct ContextPlugin;
impl Plugin for ContextPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScrollState(0))
            .add_systems(Update, (render, listen_scroll).in_set(ContextSet))
            .configure_sets(Update, ContextSet.run_if(in_state(super::Context::Initial)));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ContextSet;

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
                .block(Block::bordered().title(format!(
                    " FPS: {:.2} ",
                    diagnostics
                        .get(&FrameTimeDiagnosticsPlugin::FPS)
                        .and_then(|fps| fps.smoothed())
                        .unwrap_or(f64::NAN)
                ))),
            frame.area(),
        )
    })
    .unwrap();
}

fn listen_scroll(mut event: EventReader<Input>, mut scroll: ResMut<ScrollState>) {
    use ratatui::crossterm::event::KeyCode;
    event.read().for_each(|ev| {
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


