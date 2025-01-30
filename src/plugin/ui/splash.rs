use super::{Context, ForwardTransition, TITLE_BAR};
use crate::plugin::io::{Input, Terminal};
use bevy::prelude::*;
use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Layout},
    text::Text,
    widgets::Block,
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
            listen_input.run_if(in_state(Context::Splash)),
        ),
    );
}

fn render(mut term: ResMut<Terminal>) {
    term.draw(|frame: &mut Frame| {
        frame.render_widget(
            Text::raw(format!("{}\n{}", SPLASH, "press spacebar to continue")).centered(),
            Layout::vertical([Constraint::Ratio(1, 3); 3]).split(frame.area())[1],
        );
        frame.render_widget(Block::bordered().title(TITLE_BAR), frame.area());
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
