use bevy::prelude::*;

#[derive(Component)]
pub struct Terrain3d {
    pub height: f32,
    pub size: Vec3,
    pub subdivisions: Vec2,
}

impl Default for Terrain3d {
    fn default() -> Self {
        Self {
            height: 70.,
            size: Vec3::new(1000., 1., 1000.),
            subdivisions: Vec2::new(100., 100.),
        }
    }
}