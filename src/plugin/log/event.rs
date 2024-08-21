use bevy::prelude::*;
use std::sync::mpsc::Receiver;

#[derive(Debug, Event)]
pub struct LogEvent(pub String);

#[derive(Deref, DerefMut)]
pub struct CapturedLogEvents(pub Receiver<LogEvent>);
