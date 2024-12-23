use crate::heightmap::HeightMapPlugin;
use crate::systems::{terrain_added, toggle_wireframe};
use bevy::prelude::{App, Plugin, Update};

pub struct Terrain3dPlugin;

impl Plugin for Terrain3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (terrain_added, toggle_wireframe))
            .add_plugins(HeightMapPlugin);
    }
}