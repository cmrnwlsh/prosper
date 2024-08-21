use super::{
    event::{CapturedLogEvents, LogEvent},
    resource::LogStore,
};
use bevy::prelude::*;

pub fn store_logs(mut events: EventReader<LogEvent>, mut log_store: ResMut<LogStore>) {
    for event in events.read() {
        log_store.0.push(event.0.clone());
    }
}

pub fn transfer_log_events(
    receiver: NonSend<CapturedLogEvents>,
    mut log_events: EventWriter<LogEvent>,
) {
    log_events.send_batch(receiver.try_iter());
}
