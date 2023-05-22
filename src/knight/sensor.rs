use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::collision::{CollisionSensor, GROUND_SENSOR_GROUP};

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum KnightSensor {
    Up,
    Down,
    Back,
    Front,
    DownBack,
    DownFront,
    UpBack,
    UpFront,
}

impl CollisionSensor for KnightSensor {
    fn get_sensors() -> Vec<(Self, Collider, Vec2, CollisionGroups)> {
        vec![
            (
                Self::Up,
                Collider::cuboid(3.5, 1.0),
                Vec2::new(0.0, 13.0),
                GROUND_SENSOR_GROUP,
            ),
            (
                Self::Down,
                Collider::cuboid(3.5, 1.0),
                Vec2::new(0.0, -13.0),
                GROUND_SENSOR_GROUP,
            ),
            (
                Self::Front,
                Collider::cuboid(1.0, 11.0),
                Vec2::new(-5.5, 0.0),
                GROUND_SENSOR_GROUP,
            ),
            (
                Self::Back,
                Collider::cuboid(1.0, 11.0),
                Vec2::new(5.5, 0.0),
                GROUND_SENSOR_GROUP,
            ),
            (
                Self::DownFront,
                Collider::cuboid(1.0, 1.0),
                Vec2::new(-5.5, -13.0),
                GROUND_SENSOR_GROUP,
            ),
            (
                Self::DownBack,
                Collider::cuboid(1.0, 1.0),
                Vec2::new(5.5, -13.0),
                GROUND_SENSOR_GROUP,
            ),
            (
                Self::UpFront,
                Collider::cuboid(1.0, 1.0),
                Vec2::new(-5.5, 13.0),
                GROUND_SENSOR_GROUP,
            ),
            (
                Self::UpBack,
                Collider::cuboid(1.0, 1.0),
                Vec2::new(5.5, 13.0),
                GROUND_SENSOR_GROUP,
            ),
        ]
    }
}
