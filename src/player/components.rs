use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub move_speed: f32,
    pub acceleration: f32,
    pub jump_impulse: f32,
}
