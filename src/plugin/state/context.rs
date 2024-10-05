pub mod main {
    use bevy::{
        diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
        prelude::*,
    };
    use ratatui::{
        widgets::{Block, Paragraph},
        Frame,
    };

    use crate::plugin::{io::resource::Terminal, log::resource::LogStore};

    pub fn system(
        mut term: ResMut<Terminal>,
        diagnostics: Res<DiagnosticsStore>,
        logs: Res<LogStore>,
    ) {
        term.draw(|frame: &mut Frame| {
            frame.render_widget(
                Paragraph::new(logs.0.join("\n")).block(Block::bordered().title(format!(
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
}
