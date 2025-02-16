use crate::io::Terminal;

use super::{title_block, Context};
use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use ratatui::widgets::{Padding, Paragraph};

pub fn context(app: &mut App) {
    app.add_systems(Update, render.run_if(in_state(Context::Primary)));
}

fn render(mut term: ResMut<Terminal>, diag: Res<DiagnosticsStore>) {
    term.draw(|frame| {
        frame.render_widget(
            Paragraph::new("hello world")
                .centered()
                .block(title_block(diag).padding(Padding::top(frame.area().height / 4))),
            frame.area(),
        );
    })
    .unwrap();
}
