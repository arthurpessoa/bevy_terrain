use bevy::prelude::{App, Plugin, Update};

use crate::systems::toggle_wireframe;

pub struct Terrain3dPlugin;

impl Plugin for Terrain3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_wireframe);
    }
}