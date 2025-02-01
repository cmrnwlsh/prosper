use bevy::{
    log::{
        tracing_subscriber::{
            layer::Context, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer,
        },
        Level,
    },
    prelude::*,
    utils::tracing::{
        field::{Field, Visit},
        Event, Subscriber,
    },
};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver) = channel();
        app.insert_resource(LogStore(vec![]));
        app.insert_non_send_resource(CapturedLogEvents(receiver));
        app.add_event::<LogEvent>();
        app.add_systems(Update, (transfer_log_events, store_logs));
        registry().with(Some(CaptureLayer(sender))).init();
    }
}

#[derive(Debug, Event)]
struct LogEvent(pub String);

#[derive(Deref, DerefMut)]
struct CapturedLogEvents(pub Receiver<LogEvent>);

#[derive(Resource)]
pub struct LogStore(pub Vec<String>);

struct CaptureLayer(pub Sender<LogEvent>);
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

fn store_logs(mut events: EventReader<LogEvent>, mut log_store: ResMut<LogStore>) {
    for event in events.read() {
        log_store.0.push(event.0.clone());
    }
}

fn transfer_log_events(
    receiver: NonSend<CapturedLogEvents>,
    mut log_events: EventWriter<LogEvent>,
) {
    log_events.send_batch(receiver.try_iter());
}
