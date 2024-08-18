use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::widgets::Block;
use ratatui::{
    crossterm::event::{self},
    widgets::Paragraph,
    Frame,
};
use std::time::Duration;

use super::{event::Input, resource::Terminal};

pub fn render(mut term: ResMut<Terminal>, diagnostics: Res<DiagnosticsStore>) {
    term.draw(|frame: &mut Frame| {
        frame.render_widget(
            Paragraph::new("").block(Block::bordered().title(format!(
                "FPS: {:.2}",
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

pub fn read_events(mut commands: Commands) {
    (|| -> std::io::Result<()> {
        if event::poll(Duration::from_secs(0))? {
            if let event::Event::Key(key) = event::read()? {
                commands.add(move |w: &mut World| w.trigger(Input(key)))
            };
        }
        Ok(())
    })()
    .unwrap()
}

pub fn on_input(trigger: Trigger<Input>, mut exit: EventWriter<AppExit>) {
    match trigger.event().0 {
        KeyEvent {
            code: KeyCode::Char('c'),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::CONTROL,
            ..
        } => {
            exit.send(AppExit::Success);
        }
        _ => (),
    }
}
