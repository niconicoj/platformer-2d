use bevy::prelude::*;

use crate::{animation::Animation, collision::Collision};

use super::sensor::KnightSensor;

#[derive(Component, Eq, PartialEq, Hash, Debug)]
pub enum KnightState {
    Idle,
    Running,
    Attack,
}

pub fn update_knight_state(
    mut knight_query: Query<(
        &crate::kinematics::Velocity,
        &mut KnightState,
        &Collision<KnightSensor>,
        &Animation,
    )>,
) {
    for (velocity, mut knight_state, collisions, animation) in knight_query.iter_mut() {
        let new_state = if knight_state.eq(&KnightState::Attack) && !animation.finished {
            KnightState::Attack
        } else {
            if !knight_state.eq(&KnightState::Attack) && collisions.get(&KnightSensor::AttackZone) {
                KnightState::Attack
            } else {
                match velocity.x != 0.0 {
                    true => KnightState::Running,
                    false => KnightState::Idle,
                }
            }
        };

        println!("{:?}", new_state);

        knight_state.set_if_neq(new_state);
    }
}
