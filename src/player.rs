use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{update_animation, Animation, AnimationBundle, AnimationHandles, AnimationState},
    collision::{Collision, CollisionBundle, PLAYER_COLLIDER_GROUP},
    f32_utils::move_toward,
    kinematics::KinematicsBundle,
    GameSet,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSpritesHandles>()
            .add_startup_system(load_player_textures)
            .add_startup_system(init_player.after(load_player_textures))
            .add_system(move_player.in_set(GameSet::Update))
            .add_system(
                update_player_state
                    .in_set(GameSet::Update)
                    .after(move_player),
            )
            .add_system(
                update_animation::<PlayerState, PlayerSpritesHandles>.in_set(GameSet::AfterUpdate),
            )
            .add_system(follow_player.in_set(GameSet::AfterUpdate));
    }
}

#[derive(Component, Debug, PartialEq, Eq)]
enum PlayerState {
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

impl AnimationState for PlayerState {
    fn get_animation(&self) -> Animation {
        match self {
            PlayerState::Idle => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Repeating),
                (0..=3).into_iter().cycle(),
            ),
            PlayerState::Run => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Repeating),
                (8..=13).into_iter().cycle(),
            ),
            PlayerState::Rising => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Repeating),
                (16..=17).into_iter(),
            ),
            PlayerState::Falling => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Repeating),
                (22..=23).into_iter().cycle(),
            ),
            PlayerState::Attack1 => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 10.), TimerMode::Repeating),
                (42..=47).into_iter(),
            ),
        }
    }
}

#[derive(Resource, Default)]
struct PlayerSpritesHandles {
    handle: Handle<TextureAtlas>,
}

impl AnimationHandles<PlayerState> for PlayerSpritesHandles {
    fn get_handle(&self, _: &PlayerState) -> Handle<TextureAtlas> {
        self.handle.clone()
    }
}

#[derive(Component)]
struct Player {
    move_speed: f32,
    acceleration: f32,
    jump_impulse: f32,
}

fn load_player_textures(
    mut player_sprite_handles: ResMut<PlayerSpritesHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("adventurer/Adventurer.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 37.0), 7, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    player_sprite_handles.handle = texture_atlas_handle;
}

fn init_player(mut commands: Commands, player_sprite_handles: Res<PlayerSpritesHandles>) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
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
        CollisionBundle {
            collider: Collider::capsule_y(10.0, 4.0),
            group: PLAYER_COLLIDER_GROUP,
            ..default()
        },
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(
        &mut crate::kinematics::Velocity,
        &Player,
        &Collision,
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
        if jump && collision.down() && can_move {
            velocity.y = player.jump_impulse;
        }
        // hit the ceiling
        if !collision.down() && collision.up() && velocity.y > 0.0 {
            velocity.y = 0.0;
        }
    }
}

fn update_player_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        &crate::kinematics::Velocity,
        &Collision,
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
                match collision.down() {
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

fn follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y + 25.0;
}
