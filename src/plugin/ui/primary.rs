use crate::plugin::io::Terminal;

use super::{fps, Context, TITLE_BAR};
use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use ratatui::widgets::{Block, Padding, Paragraph};

pub fn context(app: &mut App) {
    app.add_systems(Update, render.run_if(in_state(Context::Primary)));
}

fn render(mut term: ResMut<Terminal>, diag: Res<DiagnosticsStore>) {
    term.draw(|frame| {
        let block = Block::bordered()
            .padding(Padding::top(frame.area().height / 4))
            .title(format!("{TITLE_BAR}{} ", fps(diag)));
        frame.render_widget(
            Paragraph::new("hello world").centered().block(block),
            frame.area(),
        );
    })
    .unwrap();
}
