use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::AnimationBundle,
    collision::{Collision, CollisionBundle, ENEMY_COLLIDER_GROUP},
    kinematics::KinematicsBundle,
};

use super::{
    animation::KnightSpritesHandles, components::Knight, sensor::KnightSensor, state::KnightState,
};

pub fn init_knight(mut commands: Commands, knight_sprite_handles: Res<KnightSpritesHandles>) {
    commands
        .spawn((
            RigidBody::KinematicPositionBased,
            Knight {
                move_speed: 0.5,
                direction: 1.0,
            },
            AnimationBundle::new(KnightState::Idle),
            KinematicsBundle::default(),
            SpriteSheetBundle {
                texture_atlas: knight_sprite_handles
                    .handles
                    .get(&KnightState::Idle)
                    .unwrap()
                    .clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    anchor: bevy::sprite::Anchor::Custom(Vec2::new(-0.03, -0.265)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(-30.0, -100.0, 0.0),
                    ..default()
                },
                ..default()
            },
            KinematicCharacterController {
                snap_to_ground: None,
                autostep: None,
                filter_groups: Some(ENEMY_COLLIDER_GROUP),
                ..default()
            },
            CollisionBundle::<KnightSensor> {
                collider: Collider::capsule_y(13.0, 5.5),
                ..default()
            },
        ))
        .with_children(|command| {
            command.spawn((
                Collider::cuboid(11.0, 16.0),
                Sensor,
                Transform::from_xyz(16.0, 0.0, 0.0),
            ));
        });
}

pub fn move_knight(
    mut knight_query: Query<(
        &mut crate::kinematics::Velocity,
        &mut Knight,
        &Collision<KnightSensor>,
    )>,
) {
    for (mut velocity, mut knight, collision) in knight_query.iter_mut() {
        if collision.get(&KnightSensor::Down) {
            let obstructed =
                collision.get(&KnightSensor::Front) || !collision.get(&KnightSensor::DownFront);
            if obstructed {
                knight.direction = -knight.direction;
            }

            velocity.x = knight.move_speed * knight.direction;
        }
    }
}
