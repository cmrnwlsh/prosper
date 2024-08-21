use super::event::LogEvent;
use bevy::{
    log::{
        tracing_subscriber::{layer::Context, Layer},
        Level,
    },
    utils::tracing::{
        field::{Field, Visit},
        Event, Subscriber,
    },
};
use std::sync::mpsc::Sender;

pub struct CaptureLayer(pub Sender<LogEvent>);
impl<S: Subscriber> Layer<S> for CaptureLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut message = None;
        event.record(&mut CaptureLayerVisitor(&mut message));
        if let Some(message) = message {
            let metadata = event.metadata();
            if *metadata.level() <= Level::DEBUG {
                self.0
                    .send(LogEvent(format!(
                        "[{}::{}][{}] {}",
                        metadata.target(),
                        metadata.line().unwrap_or(0),
                        metadata.level(),
                        message
                    )))
                    .unwrap();
            }
        }
    }
}

struct CaptureLayerVisitor<'a>(&'a mut Option<String>);
impl Visit for CaptureLayerVisitor<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            *self.0 = Some(format!("{value:?}"));
        }
    }
}
