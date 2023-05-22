use bevy::prelude::*;

#[derive(Component, Eq, PartialEq, Hash)]
pub enum KnightState {
    Idle,
    Running,
    Attack,
}

pub fn update_knight_state(
    mut knight_query: Query<(&crate::kinematics::Velocity, &mut KnightState)>,
) {
    for (velocity, mut knight_state) in knight_query.iter_mut() {
        let new_state = match velocity.x != 0.0 {
            true => KnightState::Running,
            false => KnightState::Idle,
        };

        knight_state.set_if_neq(new_state);
    }
}
