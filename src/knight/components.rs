use bevy::prelude::*;

#[derive(Component)]
pub struct Knight {
    pub move_speed: f32,
    pub direction: f32,
}
