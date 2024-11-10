use super::{title_block, Context};
use crate::plugin::{
    data::LoadState,
    io::{Input, Terminal},
};
use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use ratatui::{
    layout::{Constraint, Layout},
    text::Text,
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
    diagnostics: Res<DiagnosticsStore>,
    load_state: Res<State<LoadState>>,
) {
    term.draw(|frame: &mut Frame| {
        frame.render_widget(
            Text::raw(format!(
                "{}\n{}",
                SPLASH,
                if *load_state.get() == LoadState::Loaded {
                    "press any key to continue"
                } else {
                    ""
                }
            ))
            .centered(),
            Layout::vertical([Constraint::Ratio(1, 3); 3]).split(frame.area())[1],
        );
        frame.render_widget(title_block(diagnostics), frame.area());
    })
    .unwrap();
}

fn listen_input(events: EventReader<Input>, mut state: ResMut<NextState<Context>>) {
    if !events.is_empty() {
        state.set(Context::Initial)
    }
}
