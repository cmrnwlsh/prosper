use crate::resource::terminal::Terminal;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    widgets::Paragraph,
    Frame,
};
use std::time::Duration;

pub struct TuiPlugin;
impl Plugin for TuiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Terminal::init());
        app.add_systems(Update, (render, check_exit));
    }
}

fn render(mut term: ResMut<Terminal>, diagnostics: Res<DiagnosticsStore>) {
    term.draw(|frame: &mut Frame| {
        frame.render_widget(
            Paragraph::new(format!(
                "FPS: {}",
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|fps| fps.smoothed())
                    .unwrap_or(f64::NAN)
            )),
            frame.area(),
        )
    })
    .unwrap();
}

fn check_exit(mut exit: EventWriter<AppExit>) {
    (|| -> std::io::Result<()> {
        if event::poll(Duration::from_secs(0))? {
            if let event::Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) = event::read()?
            {
                exit.send(AppExit::Success);
            };
        }
        Ok(())
    })()
    .unwrap()
}
