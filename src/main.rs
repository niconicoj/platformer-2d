#![feature(trivial_bounds)]
#![feature(const_trait_impl)]
#![feature(trait_alias)]

mod animation;
mod background;
mod collision;
mod f32_utils;
mod fps;
mod kinematics;
mod knight;
mod map;
mod physics;
mod player;
mod texture_utils;

use animation::animate;
use background::BackgroundPlugin;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use fps::FpsPlugin;
use kinematics::KinematicsPlugin;
use knight::KnightPlugin;
use map::MapPlugin;
use physics::PhysicsExtensionPlugin;
use player::PlayerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameSet {
    BeforeUpdate,
    Update,
    AfterUpdate,
    Render,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(PhysicsExtensionPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(FpsPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(KinematicsPlugin)
        .add_plugin(KnightPlugin)
        .configure_sets(
            (
                GameSet::BeforeUpdate,
                GameSet::Update,
                GameSet::AfterUpdate,
                GameSet::Render,
            )
                .chain(),
        )
        .add_startup_system(setup_camera)
        .add_system(animate.in_set(GameSet::Render))
        .add_system(zoom)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        projection: OrthographicProjection {
            scale: 0.5,
            ..default()
        },
        ..default()
    });
}

fn zoom(
    mut scroll_input: EventReader<MouseWheel>,
    mut projection_query: Query<&mut OrthographicProjection>,
) {
    if let Ok(mut projection) = projection_query.get_single_mut() {
        for ev in scroll_input.iter() {
            match ev.unit {
                MouseScrollUnit::Line => {
                    projection.scale = projection.scale - ev.y * 0.02;
                }
                MouseScrollUnit::Pixel => {
                    projection.scale = projection.scale - ev.y * 0.02;
                }
            }
        }
    }
}
