use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::{kinematics::Orientation, GameSet};

use super::COLLISION_SENSOR_GROUP;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_collision_sensor.in_set(GameSet::BeforeUpdate))
            .add_system(
                flip_sensor
                    .in_set(GameSet::BeforeUpdate)
                    .after(add_collision_sensor),
            )
            .add_system(
                detect_collision
                    .in_set(GameSet::BeforeUpdate)
                    .after(flip_sensor),
            );
        // .add_system(print_collision);
    }
}

#[derive(Bundle, Default)]
pub struct CollisionBundle {
    pub collision: Collision,
    pub collider: Collider,
    pub group: CollisionGroups,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Back = 2,
    Front = 3,
    DownBack = 4,
    DownFront = 5,
    UpBack = 6,
    UpFront = 7,
}

impl Direction {
    fn get_sensor_spec(
        &self,
        collider_hx: f32,
        collider_hy: f32,
        orientation: f32,
    ) -> (f32, f32, f32, f32) {
        match self {
            Direction::Up => (0.0, collider_hy, collider_hx - 2.0, 1.0),
            Direction::Down => (0.0, -collider_hy, collider_hx - 2.0, 1.0),
            Direction::Back => (-collider_hx * orientation, 0.0, 1.0, collider_hy - 2.0),
            Direction::Front => (collider_hx * orientation, 0.0, 1.0, collider_hy - 2.0),
            Direction::DownBack => (-collider_hx * orientation, -collider_hy, 1.0, 1.0),
            Direction::UpBack => (-collider_hx * orientation, collider_hy, 1.0, 1.0),
            Direction::DownFront => (collider_hx * orientation, -collider_hy, 1.0, 1.0),
            Direction::UpFront => (collider_hx * orientation, collider_hy, 1.0, 1.0),
        }
    }
}

#[derive(Component)]
struct CollisionSensor(Direction);

#[derive(Component, Default)]
pub struct Collision {
    dimensions: (f32, f32),
    pub collisions: u8,
}

impl Collision {
    pub fn clear_collisions(&mut self) {
        self.collisions = 0;
    }

    pub fn down(&self) -> bool {
        self.collisions & (1 << Direction::Down as u8) != 0
    }

    pub fn up(&self) -> bool {
        self.collisions & (1 << Direction::Up as u8) != 0
    }

    pub fn front(&self) -> bool {
        self.collisions & (1 << Direction::Front as u8) != 0
    }

    pub fn down_front(&self) -> bool {
        self.collisions & (1 << Direction::DownFront as u8) != 0
    }

    pub fn add(&mut self, direction: Direction) {
        self.collisions |= 1 << direction as u8
    }
}

fn add_collision_sensor(
    mut commands: Commands,
    mut entity_query: Query<
        (Entity, &Collider, &mut Collision, Option<&Orientation>),
        Added<Collision>,
    >,
) {
    for (entity, collider, mut collision, orientation) in entity_query.iter_mut() {
        collision.dimensions = get_collider_dimension(collider);
        let orientation = orientation
            .map(|o| match o {
                Orientation::Right => 1.0,
                Orientation::Left => -1.0,
            })
            .unwrap_or(1.0);

        let sensor_entities: Vec<Entity> = Direction::iter()
            .map(|direction| {
                let (x, y, hx, hy) = direction.get_sensor_spec(
                    collision.dimensions.0,
                    collision.dimensions.1,
                    orientation,
                );

                commands
                    .spawn((
                        Collider::cuboid(hx, hy),
                        Transform {
                            translation: Vec3::new(x, y, 0.0),
                            ..default()
                        },
                        Sensor,
                        CollisionSensor(direction),
                        ColliderScale::Absolute(Vec2::splat(1.0)),
                        COLLISION_SENSOR_GROUP,
                        (ActiveCollisionTypes::default()
                            | ActiveCollisionTypes::KINEMATIC_STATIC
                            | ActiveCollisionTypes::STATIC_STATIC),
                    ))
                    .id()
            })
            .collect();

        commands.entity(entity).push_children(&sensor_entities);
    }
}

fn detect_collision(
    mut collision_query: Query<(&Children, &mut Collision)>,
    sensor_query: Query<(Entity, &CollisionSensor)>,
    rapier_context: Res<RapierContext>,
) {
    for (children, mut collision) in collision_query.iter_mut() {
        collision.clear_collisions();

        for &child in children.iter() {
            if let Ok((entity, sensor)) = sensor_query.get(child) {
                rapier_context
                    .intersections_with(entity)
                    .any(|(_, _, intersecting)| intersecting)
                    .then(|| collision.add(sensor.0));
            }
        }
    }
}

fn flip_sensor(
    collision_query: Query<(&Children, &Orientation, &Collision), Changed<Orientation>>,
    mut sensor_query: Query<(&mut Transform, &CollisionSensor)>,
) {
    for (children, orientation, collision) in collision_query.iter() {
        for &child in children.iter() {
            let orientation = match orientation {
                Orientation::Right => 1.0,
                Orientation::Left => -1.0,
            };
            if let Ok((mut transform, sensor)) = sensor_query.get_mut(child) {
                let (x, y, _, _) = sensor.0.get_sensor_spec(
                    collision.dimensions.0,
                    collision.dimensions.1,
                    orientation,
                );
                transform.translation = Vec3::new(x, y, transform.translation.z);
            }
        }
    }
}

fn get_collider_dimension(collider: &Collider) -> (f32, f32) {
    match collider.raw.shape_type() {
        bevy_rapier2d::rapier::prelude::ShapeType::Capsule => (
            collider.as_capsule().unwrap().radius(),
            collider.as_capsule().unwrap().half_height() + collider.as_capsule().unwrap().radius(),
        ),
        _ => panic!(
            "unsupported collider type {:?} for collision detection",
            collider.raw.shape_type()
        ),
    }
}
