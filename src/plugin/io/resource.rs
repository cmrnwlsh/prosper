use better_panic::Settings;
use bevy::prelude::*;
use delegate::delegate;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    CompletedFrame, Frame,
};
use std::{
    io::{stdout, Stdout},
    panic::set_hook,
};

#[derive(Resource)]
pub struct Terminal(ratatui::Terminal<CrosstermBackend<Stdout>>);
impl Terminal {
    pub fn init() -> Self {
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

    pub fn restore() {
        let _ = (|| -> std::io::Result<()> {
            disable_raw_mode()?;
            stdout().execute(LeaveAlternateScreen).map(|_| ())
        })();
    }

    delegate!( to self.0 {
        pub fn draw<F: FnOnce(&mut Frame<'_>)>(&mut self, render_callback: F) -> std::io::Result<CompletedFrame<'_>>;
        pub fn backend(&self) -> &CrosstermBackend<Stdout>;
        pub fn backend_mut(&mut self) -> &mut CrosstermBackend<Stdout>;
    });
}

impl Drop for Terminal {
    fn drop(&mut self) {
        Self::restore();
    }
}