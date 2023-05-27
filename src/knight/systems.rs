use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::AnimationBundle,
    collision::{Collision, CollisionBundle, ENEMY_COLLIDER_GROUP},
    f32_utils::move_toward,
    kinematics::{Gravity, KinematicsBundle},
};

use super::{
    animation::KnightSpritesHandles, components::Knight, sensor::KnightSensor, state::KnightState,
};

pub fn init_knight(mut commands: Commands, knight_sprite_handles: Res<KnightSpritesHandles>) {
    commands.spawn((
        Name::new("Knight"),
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
        ENEMY_COLLIDER_GROUP,
        CollisionBundle::<KnightSensor> {
            collider: Collider::capsule_y(13.0, 5.5),
            ..default()
        },
        Gravity,
    ));
}

pub fn move_knight(
    time: Res<Time>,
    mut knight_query: Query<(
        &mut crate::kinematics::Velocity,
        &mut Knight,
        &KnightState,
        &Collision<KnightSensor>,
    )>,
) {
    for (mut velocity, mut knight, state, collision) in knight_query.iter_mut() {
        if collision.get(&KnightSensor::Down) && !state.eq(&KnightState::Attack) {
            let obstructed =
                collision.get(&KnightSensor::Front) || !collision.get(&KnightSensor::DownFront);
            if obstructed {
                knight.direction = -knight.direction;
            }

            velocity.x = knight.move_speed * knight.direction;
        } else {
            velocity.x = move_toward(velocity.x, 0.0, 2.0, time.delta().as_secs_f32());
        }
    }
}

pub fn handle_knight_collision_changes(
    mut commands: Commands,
    mut player_query: Query<
        (
            Entity,
            Option<&mut Gravity>,
            &crate::kinematics::Velocity,
            &Collision<KnightSensor>,
        ),
        Changed<Collision<KnightSensor>>,
    >,
) {
    for (entity, gravity_marker, velocity, collisions) in player_query.iter_mut() {
        match gravity_marker {
            Some(_) => {
                if collisions.get(&KnightSensor::Down) && velocity.y <= 0.0 {
                    commands.entity(entity).remove::<Gravity>();
                }
            }
            None => {
                if !collisions.get(&KnightSensor::Down) {
                    commands.entity(entity).insert(Gravity);
                }
            }
        }
    }
}
