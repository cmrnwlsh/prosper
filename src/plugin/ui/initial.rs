use super::{title_block, Context};
use crate::plugin::{
    data::{Data, DataHandle},
    io::{Input, Terminal},
};
use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Layout},
    widgets::{Paragraph, Tabs},
    Frame,
};

pub fn context(app: &mut App) {
    app.insert_resource(State::default()).add_systems(
        Update,
        (render, listen_nav).run_if(in_state(Context::Initial)),
    );
}

#[derive(Resource, Default)]
struct State {
    tab: usize,
    scrolls: [u16; 3],
}

impl State {
    fn right(&mut self) {
        let t = &mut self.tab;
        *t = (*t + 1) % 3
    }

    fn left(&mut self) {
        let t = &mut self.tab;
        *t = if *t == 0 { 2 } else { *t - 1 }
    }

    fn up(&mut self, lines: u16) {
        let (s, t) = (&mut self.scrolls, self.tab);
        s[t] = s[t].saturating_sub(lines);
    }

    fn down(&mut self, lines: u16) {
        let (s, t) = (&mut self.scrolls, self.tab);
        s[t] = s[t].saturating_add(lines);
    }
}

fn render(
    mut term: ResMut<Terminal>,
    diagnostics: Res<DiagnosticsStore>,
    state: Res<State>,
    data_handle: Res<DataHandle>,
    data_asset: Res<Assets<Data>>,
) {
    term.draw(|frame: &mut Frame| {
        let data = data_asset.get(&data_handle.0).unwrap();
        let block = title_block(diagnostics);
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .split(block.inner(frame.area()));

        frame.render_widget(block, frame.area());
        frame.render_widget(
            Tabs::new(["items", "recipes", "buildings"]).select(state.tab),
            layout[0],
        );
        frame.render_widget(
            Paragraph::new(match state.tab {
                0 => format!("{:#?}", data.items),
                1 => format!("{:#?}", data.recipes),
                _ => format!("{:#?}", data.buildings),
            })
            .scroll((state.scrolls[state.tab], 0)),
            layout[1],
        );
    })
    .unwrap();
}

fn listen_nav(mut events: EventReader<Input>, mut state: ResMut<State>) {
    events.read().for_each(|event| match event.0.code {
        KeyCode::Right => state.right(),
        KeyCode::Left => state.left(),
        KeyCode::Up => state.up(1),
        KeyCode::Down => state.down(1),
        KeyCode::PageUp => state.up(10),
        KeyCode::PageDown => state.down(10),
        _ => {}
    });
}
