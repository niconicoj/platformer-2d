use bevy::prelude::*;

use crate::{animation::AnimationPlugin, collision::CollisionPlugin, GameSet};

use super::{
    animation::{load_knight_textures, KnightSpritesHandles},
    sensor::KnightSensor,
    state::{update_knight_state, KnightState},
    systems::{handle_knight_collision_changes, init_knight, move_knight},
};

pub struct KnightPlugin;

impl Plugin for KnightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KnightSpritesHandles>()
            .add_startup_system(load_knight_textures)
            .add_startup_system(init_knight.after(load_knight_textures))
            .add_plugin(CollisionPlugin::<KnightSensor>::default())
            .add_plugin(AnimationPlugin::<KnightState, KnightSpritesHandles>::default())
            .add_system(handle_knight_collision_changes.in_set(GameSet::Update))
            .add_system(move_knight.in_set(GameSet::Update))
            .add_system(
                update_knight_state
                    .in_set(GameSet::Update)
                    .after(move_knight),
            );
    }
}
