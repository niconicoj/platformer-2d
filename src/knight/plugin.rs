use bevy::prelude::*;

use crate::{animation::update_animation, collision::CollisionPlugin, GameSet};

use super::{
    animation::{load_knight_textures, KnightSpritesHandles},
    sensor::KnightSensor,
    state::{update_knight_state, KnightState},
    systems::{init_knight, move_knight},
};

pub struct KnightPlugin;

impl Plugin for KnightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KnightSpritesHandles>()
            .add_startup_system(load_knight_textures)
            .add_startup_system(init_knight.after(load_knight_textures))
            .add_plugin(CollisionPlugin::<KnightSensor>::default())
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
