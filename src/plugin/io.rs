use better_panic::Settings;
use bevy::prelude::*;
use delegate::delegate;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::KeyEvent,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    CompletedFrame, Frame,
};
use std::{
    io::{stdout, Stdout},
    panic::set_hook,
};

use ratatui::crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub fn io(app: &mut App) {
    app.insert_resource(Terminal::init())
        .add_event::<Input>()
        .add_systems(Update, (listen_exit, read_events));
}

#[derive(Event)]
pub struct Input(pub KeyEvent);

#[derive(Resource)]
pub struct Terminal(ratatui::Terminal<CrosstermBackend<Stdout>>);
impl Terminal {
    fn init() -> Self {
        set_hook(Box::new(move |panic_info| {
            Self::restore();
            Settings::auto()
                .most_recent_first(false)
                .lineno_suffix(true)
                .create_panic_handler()(panic_info)
        }));
        (|| -> std::io::Result<Terminal> {
            enable_raw_mode()?;
            stdout().execute(EnterAlternateScreen)?;
            Ok(Self(ratatui::Terminal::new(CrosstermBackend::new(
                stdout(),
            ))?))
        })()
        .unwrap()
    }

    fn restore() {
        let _ = (|| -> std::io::Result<()> {
            disable_raw_mode()?;
            stdout().execute(LeaveAlternateScreen).map(|_| ())
        })();
    }

    delegate!( to self.0 {
        pub fn draw<F: FnOnce(&mut Frame<'_>)>(&mut self, render_callback: F) -> std::io::Result<CompletedFrame<'_>>;
    });
}

impl Drop for Terminal {
    fn drop(&mut self) {
        Self::restore();
    }
}

fn read_events(mut event: EventWriter<Input>) {
    (|| -> std::io::Result<()> {
        if event::poll(Duration::from_secs(0))? {
            if let event::Event::Key(key) = event::read()? {
                event.send(Input(key));
            };
        }
        Ok(())
    })()
    .unwrap()
}

fn listen_exit(mut event: EventReader<Input>, mut exit: EventWriter<AppExit>) {
    event.read().for_each(|ev| {
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
