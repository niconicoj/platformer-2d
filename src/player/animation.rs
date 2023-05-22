use std::time::Duration;

use bevy::prelude::*;

use crate::animation::{Animation, AnimationHandles, AnimationState};

use super::state::PlayerState;

impl AnimationState for PlayerState {
    fn get_animation(&self) -> Animation {
        match self {
            PlayerState::Idle => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Repeating),
                (0..=3).into_iter().cycle(),
            ),
            PlayerState::Run => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Repeating),
                (8..=13).into_iter().cycle(),
            ),
            PlayerState::Rising => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Repeating),
                (16..=17).into_iter(),
            ),
            PlayerState::Falling => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 5.), TimerMode::Repeating),
                (22..=23).into_iter().cycle(),
            ),
            PlayerState::Attack1 => Animation::new(
                Timer::new(Duration::from_secs_f32(1. / 10.), TimerMode::Repeating),
                (42..=47).into_iter(),
            ),
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerSpritesHandles {
    handle: Handle<TextureAtlas>,
}

impl AnimationHandles<PlayerState> for PlayerSpritesHandles {
    fn get_handle(&self, _: &PlayerState) -> Handle<TextureAtlas> {
        self.handle.clone()
    }
}

pub fn load_player_textures(
    mut player_sprite_handles: ResMut<PlayerSpritesHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("adventurer/Adventurer.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50.0, 37.0), 7, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    player_sprite_handles.handle = texture_atlas_handle;
}
