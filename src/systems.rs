use bevy::input::ButtonInput;
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::{debug, Commands, Entity, KeyCode, Query, Res, With, Without};
use crate::components::Terrain3d;

pub fn toggle_wireframe(
    mut commands: Commands,
    landscapes_wireframes: Query<Entity, (With<Terrain3d>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<Terrain3d>, Without<Wireframe>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for terrain in &landscapes {
            commands.entity(terrain).insert(Wireframe);
            debug!("Added wireframe to terrain {:?}", terrain);
        }
        for terrain in &landscapes_wireframes {
            commands.entity(terrain).remove::<Wireframe>();
            debug!("Removed wireframe from terrain {:?}", terrain);
        }
    }
}