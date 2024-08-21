mod event;
mod layer;
pub mod resource;
mod system;

use std::sync::mpsc::channel;

use bevy::{
    app::Plugin,
    log::tracing_subscriber::{layer::SubscriberExt, registry, util::SubscriberInitExt},
    prelude::*,
};
use event::{CapturedLogEvents, LogEvent};
use layer::CaptureLayer;
use resource::LogStore;
use system::{store_logs, transfer_log_events};

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
