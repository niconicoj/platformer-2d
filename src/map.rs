use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::collision::GROUND_GROUP;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_startup_system(setup_map)
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_system(add_collider)
            .register_ldtk_int_cell::<WallBundle>(1)
            .register_ldtk_int_cell::<WallBundle>(2)
            .register_ldtk_int_cell::<WallBundle>(3);
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tileset/map.ldtk"),
        transform: Transform {
            translation: Vec3::new(-200.0, -200.0, -10.0),
            ..default()
        },
        ..default()
    });
}

fn add_collider(mut commands: Commands, wall_query: Query<Entity, Added<Wall>>) {
    for entity in wall_query.iter() {
        commands
            .entity(entity)
            .insert(Collider::cuboid(8.0, 8.0))
            .insert(GROUND_GROUP);
    }
}

#[derive(Component, Default)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}
