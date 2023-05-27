use bevy::prelude::*;

use crate::{animation::AnimationPlugin, collision::CollisionPlugin, GameSet};

use super::{
    animation::{load_player_textures, PlayerSpritesHandles},
    sensor::PlayerSensor,
    state::PlayerState,
    systems::{
        follow_player, handle_player_collision_changes, init_player, move_player,
        update_player_state,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSpritesHandles>()
            .add_startup_system(load_player_textures)
            .add_startup_system(init_player.after(load_player_textures))
            .add_plugin(CollisionPlugin::<PlayerSensor>::default())
            .add_plugin(AnimationPlugin::<PlayerState, PlayerSpritesHandles>::default())
            .add_system(
                move_player
                    .in_set(GameSet::Update)
                    .before(update_player_state),
            )
            .add_system(
                handle_player_collision_changes
                    .in_set(GameSet::Update)
                    .before(update_player_state),
            )
            .add_system(update_player_state.in_set(GameSet::Update))
            .add_system(follow_player.in_set(GameSet::AfterUpdate));
    }
}
