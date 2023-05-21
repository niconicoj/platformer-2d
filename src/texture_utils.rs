use std::hash::Hash;

use bevy::{prelude::*, utils::HashMap};

pub fn register_texture_atlas<T: Eq + PartialEq + Hash>(
    key: T,
    asset_path: &str,
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    handles: &mut HashMap<T, Handle<TextureAtlas>>,
) {
    let idle_texture_handle = asset_server.load(asset_path);
    let idle_texture_atlas =
        TextureAtlas::from_grid(idle_texture_handle, tile_size, columns, rows, None, None);
    let idle_texture_atlas_handle = texture_atlases.add(idle_texture_atlas);
    handles.insert(key, idle_texture_atlas_handle);
}
