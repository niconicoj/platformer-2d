use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{Animation, AnimationBundle, AnimationHandles},
    collision::{Collision, CollisionBundle, PLAYER_COLLIDER_GROUP},
    f32_utils::move_toward,
    kinematics::{Gravity, KinematicsBundle},
};

use super::{
    animation::PlayerSpritesHandles, components::Player, sensor::PlayerSensor, state::PlayerState,
};

pub fn init_player(mut commands: Commands, player_sprite_handles: Res<PlayerSpritesHandles>) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Gravity,
        Player {
            move_speed: 1.5,
            acceleration: 8.0,
            jump_impulse: 4.5,
        },
        AnimationBundle::new(PlayerState::Idle),
        KinematicsBundle::default(),
        SpriteSheetBundle {
            texture_atlas: player_sprite_handles.get_handle(&PlayerState::Idle),
            sprite: TextureAtlasSprite {
                index: 0,
                anchor: bevy::sprite::Anchor::Custom(Vec2::new(0., -0.09)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(82.0, -70.0, 0.0),
                ..default()
            },
            ..default()
        },
        KinematicCharacterController {
            snap_to_ground: None,
            autostep: None,
            filter_groups: Some(PLAYER_COLLIDER_GROUP),
            ..default()
        },
        CollisionBundle::<PlayerSensor> {
            collider: Collider::capsule_y(10.0, 4.0),
            ..default()
        },
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(
        &mut crate::kinematics::Velocity,
        &Player,
        &Collision<PlayerSensor>,
        &PlayerState,
    )>,
) {
    for (mut velocity, player, collision, player_state) in player_query.iter_mut() {
        let can_move = player_state.can_move();
        let direction = ((keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) as i8)
            - (keyboard_input.any_pressed([KeyCode::Left, KeyCode::Q]) as i8))
            as f32
            * (can_move as i8 as f32);

        velocity.x = move_toward(
            velocity.x,
            player.move_speed * direction,
            player.acceleration,
            time.delta_seconds(),
        );

        let jump = keyboard_input.any_just_pressed([KeyCode::Up, KeyCode::Z]);
        if jump && collision.get(&PlayerSensor::Down) && can_move {
            velocity.y = player.jump_impulse;
        }
        // hit the ceiling
        if !collision.get(&PlayerSensor::Down)
            && collision.get(&PlayerSensor::Up)
            && velocity.y > 0.0
        {
            velocity.y = 0.0;
        }
    }
}

pub fn update_player_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        &crate::kinematics::Velocity,
        &Collision<PlayerSensor>,
        &mut PlayerState,
        &Animation,
    )>,
) {
    if let Ok((velocity, collision, mut player_state, player_animation)) =
        player_query.get_single_mut()
    {
        let new_state = match velocity.y > 0.0 {
            true => PlayerState::Rising,
            false => {
                match collision.get(&PlayerSensor::Down) {
                    true => {
                        if player_state.eq(&PlayerState::Attack1) && !player_animation.finished {
                            PlayerState::Attack1
                        } else {
                            if keyboard_input.just_pressed(KeyCode::Space) {
                                // TODO : need to orient the character in the direction the player is
                                // aiming to
                                PlayerState::Attack1
                            } else {
                                match velocity.x == 0.0 {
                                    true => PlayerState::Idle,
                                    false => PlayerState::Run,
                                }
                            }
                        }
                    }
                    false => PlayerState::Falling,
                }
            }
        };

        player_state.set_if_neq(new_state);
    }
}

pub fn handle_player_collision_changes(
    mut commands: Commands,
    mut player_query: Query<
        (
            Entity,
            Option<&mut Gravity>,
            &crate::kinematics::Velocity,
            &Collision<PlayerSensor>,
        ),
        Changed<Collision<PlayerSensor>>,
    >,
) {
    for (entity, gravity_marker, velocity, collisions) in player_query.iter_mut() {
        match gravity_marker {
            Some(_) => {
                if collisions.get(&PlayerSensor::Down) && velocity.y <= 0.0 {
                    commands.entity(entity).remove::<Gravity>();
                }
            }
            None => {
                if !collisions.get(&PlayerSensor::Down) {
                    commands.entity(entity).insert(Gravity);
                }
            }
        }
    }
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y + 25.0;
}
