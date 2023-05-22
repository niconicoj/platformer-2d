use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Run,
    Rising,
    Falling,
    Attack1,
}

impl PlayerState {
    pub fn can_move(&self) -> bool {
        match self {
            PlayerState::Attack1 => false,
            _ => true,
        }
    }
}
