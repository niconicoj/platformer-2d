use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameSet;

const GRAVITY: f32 = -9.81;

pub struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_gravity.in_set(GameSet::BeforeUpdate))
            .add_system(cancel_gravity.in_set(GameSet::AfterUpdate))
            .add_system(update_character_orientations.in_set(GameSet::AfterUpdate))
            .add_system(
                update_characters_positions
                    .in_set(GameSet::AfterUpdate)
                    .after(cancel_gravity),
            );
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

#[derive(Component)]
pub struct Gravity;

fn apply_gravity(time: Res<Time>, mut gravity_query: Query<&mut Velocity, With<Gravity>>) {
    for mut velocity in gravity_query.iter_mut() {
        velocity.y += GRAVITY * time.delta_seconds();
    }
}

fn cancel_gravity(
    mut removed_gravity: RemovedComponents<Gravity>,
    mut velocity_query: Query<&mut Velocity>,
) {
    for entity in removed_gravity.iter() {
        if let Ok(mut velocity) = velocity_query.get_mut(entity) {
            velocity.y = 0.0;
        }
    }
}

fn update_character_orientations(
    mut character_query: Query<
        (&mut Orientation, &mut Transform, &Velocity),
        With<KinematicCharacterController>,
    >,
) {
    for (mut facing_direction, mut transform, velocity) in character_query.iter_mut() {
        if velocity.x != 0.0 {
            let new_direction = match velocity.x.is_sign_positive() {
                true => Orientation::Right,
                false => Orientation::Left,
            };

            transform.scale.x = velocity.x.signum();

            facing_direction.set_if_neq(new_direction);
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
