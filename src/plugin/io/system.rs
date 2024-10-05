use super::event::Input;
use bevy::prelude::*;
use ratatui::crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

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
        ev => info!("{:#?}", ev),
    }
}
