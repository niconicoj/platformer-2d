use std::marker::PhantomData;

use bevy::prelude::*;

use crate::GameSet;

pub trait AnimationStateComponent = AnimationState + Component + Send + Sync + 'static;
pub trait AnimationHandlesComponent<T: AnimationState> =
    AnimationHandles<T> + Resource + Default + Send + Sync + 'static;

pub struct AnimationPlugin<T: AnimationStateComponent, U: AnimationHandlesComponent<T>> {
    phantom_t: PhantomData<T>,
    phantom_u: PhantomData<U>,
}

impl<T: AnimationStateComponent, U: AnimationHandlesComponent<T>> Default
    for AnimationPlugin<T, U>
{
    fn default() -> Self {
        Self {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        }
    }
}

impl<T: AnimationStateComponent, U: AnimationHandlesComponent<T>> Plugin for AnimationPlugin<T, U> {
    fn build(&self, app: &mut App) {
        app.init_resource::<U>()
            .add_system(update_animation::<T, U>.in_set(GameSet::AfterUpdate));
    }
}

#[derive(Bundle)]
pub struct AnimationBundle<T: AnimationState + Component> {
    animation: Animation,
    state: T,
}

impl<T: AnimationState + Component> AnimationBundle<T> {
    pub fn new(initial_state: T) -> Self {
        Self {
            animation: initial_state.get_animation(),
            state: initial_state,
        }
    }
}

pub trait AnimationState {
    fn get_animation(&self) -> Animation;
}

pub trait AnimationHandles<T: AnimationState> {
    fn get_handle(&self, key: &T) -> Handle<TextureAtlas>;
}

#[derive(Component)]
pub struct Animation {
    timer: Timer,
    frame_iter: Box<dyn Iterator<Item = usize> + Send + Sync>,
    pub finished: bool,
}

impl Animation {
    pub fn new(
        timer: Timer,
        frame_iter: impl Iterator<Item = usize> + Send + Sync + 'static,
    ) -> Self {
        Self {
            timer,
            frame_iter: Box::new(frame_iter),
            finished: false,
        }
    }
}

pub fn update_animation<T: AnimationState + Component, U: AnimationHandles<T> + Resource>(
    animation_handles: Res<U>,
    mut animation_query: Query<
        (
            &mut Animation,
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
            &T,
        ),
        Changed<T>,
    >,
) {
    for (mut animation, mut texture_atlas_sprite, mut texture_atlas_handle, animation_state) in
        animation_query.iter_mut()
    {
        *animation = animation_state.get_animation();
        *texture_atlas_handle = animation_handles.get_handle(animation_state);
        if let Some(next_index) = animation.frame_iter.next() {
            texture_atlas_sprite.index = next_index;
        } else {
            animation.finished = true;
        }
    }
}

pub fn animate(
    time: Res<Time>,
    mut animation_query: Query<(&mut Animation, &mut TextureAtlasSprite)>,
) {
    for (mut animation, mut texture_atlas_sprite) in animation_query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            if let Some(next_index) = animation.frame_iter.next() {
                texture_atlas_sprite.index = next_index;
            } else {
                animation.finished = true;
            }
        }
    }
}
