use bevy::prelude::*;

pub struct SimPlugin;
impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Position { x: 25, y: 25 });
        });
    }
}

#[derive(Component)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Component)]
pub struct Symbol(char);
