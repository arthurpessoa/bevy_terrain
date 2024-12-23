mod asset_loader;
mod mesh_builder;
use mesh_builder::MeshBuilder;

use crate::heightmap::asset_loader::HeightMapLoader;
use bevy::prelude::*;

pub struct HeightMap<H: Fn(Vec2) -> f32> {
    pub size: UVec2,
    pub h: H,
}

impl<H: Fn(Vec2) -> f32> From<HeightMap<H>> for Mesh {
    fn from(HeightMap { size, h }: HeightMap<H>) -> Self {
        MeshBuilder::grid(size, &h).build()
    }
}


pub struct HeightMapPlugin;
impl Plugin for HeightMapPlugin {
    fn build(&self, app: &mut App) {
        app.preregister_asset_loader::<HeightMapLoader>(HeightMapLoader::EXTENSIONS);
    }
    fn finish(&self, app: &mut App) {
        app.init_asset_loader::<HeightMapLoader>();
    }
}