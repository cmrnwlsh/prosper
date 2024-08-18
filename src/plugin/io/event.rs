use bevy::prelude::*;
use ratatui::crossterm::event::KeyEvent;

#[derive(Event)]
pub struct Input(pub KeyEvent);
