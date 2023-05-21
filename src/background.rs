use bevy::prelude::*;

use crate::GameSet;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(init_background)
            .add_system(display_backgrounds.in_set(GameSet::Render))
            .add_system(
                update_parallax
                    .in_set(GameSet::Render)
                    .after(display_backgrounds),
            );
    }
}

const FAR_CLIP_PLANE: f32 = -999.9;
const BACKGROUND_SPRITE_WIDTH: f32 = 688.0;
const LOOP_SAFETY_MARGIN: f32 = 50.0;

#[derive(Component, Default)]
struct ParallaxBackground {
    anchor: Vec2,
    z_index: f32,
    scale: f32,
    texture: Handle<Image>,
}

#[derive(Bundle, Default)]
struct ParallaxBackgroundBundle {
    parallax_background: ParallaxBackground,
    spatial_bundle: SpatialBundle,
}

#[derive(Component)]
struct ParallaxBackgroundSprite {
    anchor: Vec2,
    index: isize,
}

fn init_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(ParallaxBackgroundBundle {
        parallax_background: ParallaxBackground {
            anchor: Vec2::new(0.0, 15.0),
            z_index: -999.9,
            scale: 1.0,
            texture: asset_server.load("background/sky_cloud.png"),
        },
        ..default()
    });

    commands.spawn(ParallaxBackgroundBundle {
        parallax_background: ParallaxBackground {
            anchor: Vec2::new(0.0, -80.0),
            z_index: -900.0,
            scale: 1.0,
            texture: asset_server.load("background/mountain2.png"),
        },
        ..default()
    });

    commands.spawn(ParallaxBackgroundBundle {
        parallax_background: ParallaxBackground {
            anchor: Vec2::new(0.0, -80.0),
            z_index: -200.0,
            scale: 1.0,
            texture: asset_server.load("background/pine2.png"),
        },
        ..default()
    });

    commands.spawn(ParallaxBackgroundBundle {
        parallax_background: ParallaxBackground {
            anchor: Vec2::new(0.0, -160.0),
            z_index: -20.0,
            scale: 1.0,
            texture: asset_server.load("background/pine1.png"),
        },
        ..default()
    });
}

fn display_backgrounds(
    mut commands: Commands,
    parallax_backgrounds: Query<(Entity, &ParallaxBackground)>,
    children_query: Query<&Children>,
    background_sprites_query: Query<&ParallaxBackgroundSprite>,
    camera: Query<
        (&Transform, &OrthographicProjection),
        (With<Camera2d>, Without<ParallaxBackground>),
    >,
) {
    let (camera_transform, orthographic_projection) = camera.single();

    let left_clip_x = camera_transform.translation.x + orthographic_projection.area.min.x;
    let right_clip_x = camera_transform.translation.x + orthographic_projection.area.max.x;

    for (entity, parallax_background) in parallax_backgrounds.iter() {
        let mut visible_indexes =
            get_visible_indexes(left_clip_x, right_clip_x, parallax_background.z_index);

        for descendant in children_query.iter_descendants(entity) {
            if let Ok(background_sprite) = background_sprites_query.get(descendant) {
                visible_indexes.retain(|&i| i != background_sprite.index);
            }
        }

        for index in visible_indexes {
            let new_background_entity = commands
                .spawn((
                    ParallaxBackgroundSprite {
                        index,
                        anchor: parallax_background.anchor,
                    },
                    SpriteBundle {
                        texture: parallax_background.texture.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                BACKGROUND_SPRITE_WIDTH * (index as f32) * 2.,
                                parallax_background.anchor.y,
                                parallax_background.z_index,
                            ),
                            scale: Vec3::splat(parallax_background.scale),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .id();
            commands
                .entity(entity)
                .push_children(&[new_background_entity]);
        }
    }
}

fn get_visible_indexes(left_clip: f32, right_clip: f32, z_index: f32) -> Vec<isize> {
    let nearest_left = ((left_clip - LOOP_SAFETY_MARGIN) / BACKGROUND_SPRITE_WIDTH
        * (FAR_CLIP_PLANE - z_index)
        / FAR_CLIP_PLANE)
        .round();
    let nearest_right = ((right_clip + LOOP_SAFETY_MARGIN) / BACKGROUND_SPRITE_WIDTH
        * (FAR_CLIP_PLANE - z_index)
        / FAR_CLIP_PLANE)
        .round();

    ((nearest_left as isize)..=(nearest_right as isize))
        .into_iter()
        .collect()
}

fn update_parallax(
    mut parallax_backgrounds: Query<(&mut Transform, &ParallaxBackgroundSprite)>,
    camera: Query<&Transform, (With<Camera2d>, Without<ParallaxBackgroundSprite>)>,
) {
    let camera = camera.single();

    for (mut transform, parallax_background) in parallax_backgrounds.iter_mut() {
        let z_value = transform.translation.z;

        transform.translation = Vec3::new(
            (parallax_background.index as f32) * (BACKGROUND_SPRITE_WIDTH)
                + camera.translation.x * (z_value / FAR_CLIP_PLANE),
            parallax_background.anchor.y + camera.translation.y * (z_value / FAR_CLIP_PLANE),
            transform.translation.z,
        );
    }
}
