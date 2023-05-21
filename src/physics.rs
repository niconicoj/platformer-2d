use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::Isometry};

pub struct PhysicsExtensionPlugin;

impl Plugin for PhysicsExtensionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_child_collider_transform_changes.in_base_set(PhysicsSet::Writeback));
    }
}

fn apply_child_collider_transform_changes(
    mut context: ResMut<RapierContext>,
    collider_query: Query<
        (&Parent, &RapierColliderHandle, &Transform),
        (
            Without<RapierRigidBodyHandle>,
            Without<GlobalTransform>,
            Changed<Transform>,
        ),
    >,
    parent_query: Query<Entity, With<RigidBody>>,
) {
    for (parent, handle, transform) in collider_query.iter() {
        let scale = context.physics_scale();
        if let Some(co) = context.colliders.get_mut(handle.0) {
            if let Ok(_) = parent_query.get(parent.get()) {
                co.set_position_wrt_parent(transform_to_iso(&transform, scale));
            }
        }
    }
}

fn transform_to_iso(transform: &Transform, physics_scale: Real) -> Isometry<Real> {
    use bevy::math::Vec3Swizzles;
    Isometry::new(
        (transform.translation / physics_scale).xy().into(),
        transform.rotation.to_scaled_axis().z,
    )
}
