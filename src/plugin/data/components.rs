use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Symbol(pub char);
