use bevy::app::Startup;
use bevy::prelude::{App, Plugin, Update};

use crate::systems::{terrain_added, toggle_wireframe};

pub struct Terrain3dPlugin;

impl Plugin for Terrain3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (terrain_added, toggle_wireframe));
    }
}