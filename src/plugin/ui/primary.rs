use crate::plugin::{
    io::Terminal,
    map::TileMap,
    sim::components::{Position, Symbol},
};

use super::{Context, TITLE_BAR};
use bevy::prelude::*;
use ratatui::widgets::{Block, Padding, Paragraph};

pub fn context(app: &mut App) {
    app.add_systems(Update, render.run_if(in_state(Context::Primary)));
}

fn render(mut term: ResMut<Terminal>, map: Res<TileMap>, symbols: Query<(&Position, &Symbol)>) {
    let mut out = map.symbol_grid();
    symbols.into_iter().for_each(|(Position { x, y }, symbol)| {
        out[*y][*x] = symbol.0;
    });
    let out = out
        .into_iter()
        .map(|line| line.into_iter().collect())
        .collect::<Vec<String>>()
        .join("\n");
    term.draw(|frame| {
        let block = Block::bordered()
            .padding(Padding::top(frame.area().height / 4))
            .title(TITLE_BAR);
        frame.render_widget(Paragraph::new(out).centered().block(block), frame.area());
    })
    .unwrap();
}
