use std::{hash::Hash, time::Duration};

use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    animation::{update_animation, Animation, AnimationBundle, AnimationHandles, AnimationState},
    collision::{Collision, CollisionBundle, ENEMY_COLLIDER_GROUP},
    kinematics::KinematicsBundle,
    texture_utils::register_texture_atlas,
    GameSet,
};

pub struct KnightPlugin;

impl Plugin for KnightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KnightSpritesHandles>()
            .add_startup_system(load_knight_textures)
            .add_startup_system(init_knight.after(load_knight_textures))
            .add_system(move_knight.in_set(GameSet::Update))
            .add_system(
                update_knight_state
                    .in_set(GameSet::Update)
                    .after(move_knight),
            )
            .add_system(
                update_animation::<KnightState, KnightSpritesHandles>.in_set(GameSet::AfterUpdate),
            );
    }
}

#[derive(Component, Eq, PartialEq, Hash)]
enum KnightState {
    Idle,
    Running,
}

#[derive(Resource, Default)]
struct KnightSpritesHandles {
    handles: HashMap<KnightState, Handle<TextureAtlas>>,
}

impl AnimationHandles<KnightState> for KnightSpritesHandles {
    fn get_handle(&self, key: &KnightState) -> Handle<TextureAtlas> {
        self.handles.get(key).unwrap().clone()
    }
}

fn load_knight_textures(
    mut knight_sprite_handles: ResMut<KnightSpritesHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    register_texture_atlas(
        KnightState::Idle,
        "knight/_Idle.png",
        Vec2::new(120.0, 80.0),
        10,
        1,
        &asset_server,
        &mut texture_atlases,
        &mut knight_sprite_handles.handles,
    );

    register_texture_atlas(
        KnightState::Running,
        "knight/_Run.png",
        Vec2::new(120.0, 80.0),
        10,
        1,
        &asset_server,
        &mut texture_atlases,
        &mut knight_sprite_handles.handles,
    );
}

impl AnimationState for KnightState {
    fn get_animation(&self) -> crate::animation::Animation {
        match self {
            _ => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 10.), TimerMode::Repeating),
                (0..=9).into_iter().cycle(),
            ),
        }
    }
}

#[derive(Component)]
pub struct Knight {
    move_speed: f32,
    direction: f32,
}

fn init_knight(mut commands: Commands, knight_sprite_handles: Res<KnightSpritesHandles>) {
    commands.spawn((
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
        CollisionBundle {
            collider: Collider::capsule_y(13.0, 5.5),
            group: ENEMY_COLLIDER_GROUP,
            ..default()
        },
    ));
}

fn move_knight(
    mut knight_query: Query<(&mut crate::kinematics::Velocity, &mut Knight, &Collision)>,
) {
    for (mut velocity, mut knight, collision) in knight_query.iter_mut() {
        if collision.down() {
            let obstructed = collision.front() || !collision.down_front();
            if obstructed {
                knight.direction = -knight.direction;
            }

            velocity.x = knight.move_speed * knight.direction;
        }
    }
}

fn update_knight_state(mut knight_query: Query<(&crate::kinematics::Velocity, &mut KnightState)>) {
    for (velocity, mut knight_state) in knight_query.iter_mut() {
        let new_state = match velocity.x != 0.0 {
            true => KnightState::Running,
            false => KnightState::Idle,
        };

        knight_state.set_if_neq(new_state);
    }
}
