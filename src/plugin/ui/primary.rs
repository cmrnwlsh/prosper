use crate::plugin::{
    io::Terminal,
    sim::components::{Position, Symbol},
};

use super::{Context, TITLE_BAR};
use bevy::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph};

pub fn context(app: &mut App) {
    app.add_systems(Update, render.run_if(in_state(Context::Primary)));
}

fn render(mut term: ResMut<Terminal>, symbols: Query<(&Position, &Symbol)>) {
    term.draw(|frame| {
        let block = Block::bordered()
            .padding(Padding::top(frame.area().height / 4))
            .title(TITLE_BAR);
        frame.render_widget(
            Paragraph::new("hello world").centered().block(block),
            frame.area(),
        );
    })
    .unwrap();
}
