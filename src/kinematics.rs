use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{collision::Collision, GameSet};

const GRAVITY: f32 = -9.81;

pub struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_gravity.in_set(GameSet::BeforeUpdate))
            .add_system(update_character_orientations.in_set(GameSet::AfterUpdate))
            .add_system(update_characters_positions.in_set(GameSet::AfterUpdate));
    }
}

#[derive(Bundle, Default)]
pub struct KinematicsBundle {
    velocity: Velocity,
    orientation: Orientation,
}

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default, Eq, PartialEq, Debug)]
pub enum Orientation {
    #[default]
    Right,
    Left,
}

fn apply_gravity(time: Res<Time>, mut collision_query: Query<(&mut Velocity, &Collision)>) {
    for (mut velocity, collision) in collision_query.iter_mut() {
        if !collision.down() {
            velocity.y += GRAVITY * time.delta_seconds();
        } else {
            if velocity.y <= 0.0 {
                velocity.y = -0.5;
            }
        }
    }
}

fn update_character_orientations(
    mut character_query: Query<
        (
            &mut Orientation,
            &mut Transform,
            Option<&mut Sprite>,
            Option<&mut TextureAtlasSprite>,
            &Velocity,
        ),
        With<KinematicCharacterController>,
    >,
) {
    for (mut facing_direction, mut transform, sprite, texture_atlas_sprite, velocity) in
        character_query.iter_mut()
    {
        if velocity.x != 0.0 {
            let new_direction = match velocity.x.is_sign_positive() {
                true => Orientation::Right,
                false => Orientation::Left,
            };

            transform.scale.x = velocity.x.signum();

            facing_direction.set_if_neq(new_direction);

            // if let Some(mut sprite) = sprite {
            //     println!("sprite flipped : {:?}", velocity.x.is_sign_positive());
            //     sprite.flip_x = velocity.x.is_sign_negative();
            // }

            // if let Some(mut texture_atlas_sprite) = texture_atlas_sprite {
            //     texture_atlas_sprite.flip_x = velocity.x.is_sign_negative();
            // }
        }
    }
}

fn update_characters_positions(
    mut controllers: Query<(&mut KinematicCharacterController, &Velocity)>,
) {
    for (mut controller, velocity) in controllers.iter_mut() {
        controller.translation = Some(Vec2::new(velocity.x, velocity.y));
    }
}
