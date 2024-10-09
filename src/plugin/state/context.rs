use bevy::{app::PluginGroupBuilder, prelude::*};
use initial::Initial;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Context {
    Initial,
}

pub struct ContextPlugin;
impl PluginGroup for ContextPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(Initial)
    }
}

pub mod initial {
    use crate::plugin::{
        io::{Input, Terminal},
        log::LogStore,
    };
    use bevy::{
        diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
        prelude::*,
    };
    use ratatui::{
        widgets::{Block, Paragraph},
        Frame,
    };

    #[derive(Resource)]
    struct ScrollState(u16);

    pub struct Initial;
    impl Plugin for Initial {
        fn build(&self, app: &mut App) {
            app.add_systems(
                Update,
                (render, listen_scroll).run_if(in_state(super::Context::Initial)),
            )
            .insert_resource(ScrollState(0));
        }
    }

    fn render(
        mut term: ResMut<Terminal>,
        diagnostics: Res<DiagnosticsStore>,
        logs: Res<LogStore>,
        scroll: Res<ScrollState>,
    ) {
        term.draw(|frame: &mut Frame| {
            frame.render_widget(
                Paragraph::new(logs.0.join("\n"))
                    .scroll((scroll.0, 0))
                    .block(Block::bordered().title(format!(
                    "FPS: {:.2}",
                    diagnostics
                        .get(&FrameTimeDiagnosticsPlugin::FPS)
                        .and_then(|fps| fps.smoothed())
                        .unwrap_or(f64::NAN)
                ))),
                frame.area(),
            )
        })
        .unwrap();
    }

    fn listen_scroll(mut event: EventReader<Input>, mut scroll: ResMut<ScrollState>) {
        use ratatui::crossterm::event::KeyCode;
        event.read().for_each(|ev| {
            scroll.0 = match ev.0.code {
                KeyCode::Up => scroll.0.saturating_sub(1),
                KeyCode::Down => scroll.0.saturating_add(1),
                _ => scroll.0,
            }
        })
    }
}
