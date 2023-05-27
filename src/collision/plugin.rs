use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;

use crate::{kinematics::Orientation, GameSet};

pub trait CollisionSensorComponent =
    CollisionSensor + Component + Debug + Copy + Eq + PartialEq + Hash + Send + Sync + 'static;

pub struct CollisionPlugin<T> {
    phantom: PhantomData<T>,
}

impl<T: CollisionSensorComponent> Default for CollisionPlugin<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T: CollisionSensorComponent + Debug> Plugin for CollisionPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(add_collision_sensor::<T>.in_set(GameSet::BeforeUpdate))
            .add_system(
                flip_sensor::<T>
                    .in_set(GameSet::BeforeUpdate)
                    .after(add_collision_sensor::<T>),
            )
            .add_system(
                detect_collision::<T>
                    .in_set(GameSet::BeforeUpdate)
                    .after(flip_sensor::<T>),
            );
    }
}

pub trait CollisionSensor: Sized {
    fn get_sensors() -> Vec<(Self, Collider, Vec2, CollisionGroups)>;
}

#[derive(Bundle)]
pub struct CollisionBundle<T: CollisionSensorComponent> {
    pub collider: Collider,
    pub collisions: Collision<T>,
}

impl<T: CollisionSensorComponent> Default for CollisionBundle<T> {
    fn default() -> Self {
        Self {
            collider: Collider::default(),
            collisions: Collision::default(),
        }
    }
}

#[derive(Component)]
pub struct Collision<T> {
    pub collisions: HashSet<T>,
}

impl<T: CollisionSensorComponent> Default for Collision<T> {
    fn default() -> Self {
        Self {
            collisions: HashSet::new(),
        }
    }
}

impl<T: Eq + PartialEq + Hash> Collision<T> {
    pub fn add(&mut self, sensor: T) {
        self.collisions.insert(sensor);
    }

    pub fn remove(&mut self, sensor: &T) {
        self.collisions.remove(sensor);
    }

    pub fn get(&self, sensor: &T) -> bool {
        self.collisions.contains(sensor)
    }
}

impl<T: CollisionSensor> Collision<T> {
    pub fn get_sensors(&self) -> Vec<(T, Collider, Vec2, CollisionGroups)> {
        T::get_sensors()
    }
}

pub fn add_collision_sensor<T: CollisionSensorComponent>(
    mut commands: Commands,
    mut entity_query: Query<(Entity, &mut Collision<T>), Added<Collision<T>>>,
) {
    for (entity, collision) in entity_query.iter_mut() {
        let sensor_entities: Vec<Entity> = collision
            .get_sensors()
            .into_iter()
            .map(|(sensor, collider, relative_position, collision_group)| {
                commands
                    .spawn((
                        Name::new(format!("{:?} sensor", sensor)),
                        collider,
                        sensor,
                        collision_group,
                        Transform {
                            translation: Vec3::new(relative_position.x, relative_position.y, 0.0),
                            ..default()
                        },
                        Sensor,
                        ColliderScale::Absolute(Vec2::splat(1.0)),
                        (ActiveCollisionTypes::default()
                            | ActiveCollisionTypes::KINEMATIC_KINEMATIC
                            | ActiveCollisionTypes::KINEMATIC_STATIC
                            | ActiveCollisionTypes::STATIC_STATIC),
                    ))
                    .id()
            })
            .collect();

        commands.entity(entity).push_children(&sensor_entities);
    }
}

fn detect_collision<T: CollisionSensorComponent + Debug>(
    mut collision_query: Query<(&Children, &mut Collision<T>)>,
    sensor_query: Query<(Entity, &T)>,
    rapier_context: Res<RapierContext>,
) {
    for (children, mut collision) in collision_query.iter_mut() {
        for &child in children.iter() {
            if let Ok((entity, sensor)) = sensor_query.get(child) {
                let colliding = rapier_context
                    .intersections_with(entity)
                    .any(|(_, _, intersecting)| intersecting);
                match colliding {
                    true => {
                        if !collision.get(sensor) {
                            collision.add(*sensor);
                        }
                    }
                    false => {
                        if collision.get(sensor) {
                            collision.remove(sensor);
                        }
                    }
                }
            }
        }
    }
}

fn flip_sensor<T: CollisionSensorComponent>(
    collision_query: Query<&Children, Changed<Orientation>>,
    mut sensor_query: Query<&mut Transform, With<T>>,
) {
    for children in collision_query.iter() {
        for &child in children.iter() {
            if let Ok(mut transform) = sensor_query.get_mut(child) {
                transform.translation.x = -transform.translation.x;
            }
        }
    }
}
