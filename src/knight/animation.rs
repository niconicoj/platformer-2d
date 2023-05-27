use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};

use crate::{
    animation::{Animation, AnimationHandles, AnimationState},
    texture_utils::register_texture_atlas,
};

use super::state::KnightState;

#[derive(Resource, Default)]
pub struct KnightSpritesHandles {
    pub handles: HashMap<KnightState, Handle<TextureAtlas>>,
}

impl AnimationHandles<KnightState> for KnightSpritesHandles {
    fn get_handle(&self, key: &KnightState) -> Handle<TextureAtlas> {
        self.handles.get(key).unwrap().clone()
    }
}

pub fn load_knight_textures(
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

    register_texture_atlas(
        KnightState::Attack,
        "knight/_Attack.png",
        Vec2::new(120.0, 80.0),
        4,
        1,
        &asset_server,
        &mut texture_atlases,
        &mut knight_sprite_handles.handles,
    );
}

impl AnimationState for KnightState {
    fn get_animation(&self) -> crate::animation::Animation {
        match self {
            Self::Attack => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Repeating),
                [0, 0, 0, 1, 2, 3].into_iter(),
            ),
            _ => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 10.), TimerMode::Repeating),
                (0..=9).into_iter().cycle(),
            ),
        }
    }
}
