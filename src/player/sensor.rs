use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::collision::{CollisionSensor, GROUND_SENSOR_GROUP, PLAYER_HITBOX_GROUP};

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PlayerSensor {
    Up,
    Down,
    Hitbox,
}

impl CollisionSensor for PlayerSensor {
    fn get_sensors() -> Vec<(Self, Collider, Vec2, CollisionGroups)> {
        vec![
            (
                Self::Up,
                Collider::cuboid(3.0, 1.0),
                Vec2::new(0.0, 14.0),
                GROUND_SENSOR_GROUP,
            ),
            (
                Self::Down,
                Collider::cuboid(3.0, 1.0),
                Vec2::new(0.0, -14.0),
                GROUND_SENSOR_GROUP,
            ),
            (
                Self::Hitbox,
                Collider::capsule_y(10.0, 4.0),
                Vec2::new(0.0, 0.0),
                PLAYER_HITBOX_GROUP,
            ),
        ]
    }
}
