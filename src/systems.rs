use bevy::input::ButtonInput;
use bevy::pbr::wireframe::Wireframe;
use bevy::prelude::{debug, Added, Assets, Color, Commands, Entity, KeyCode, Mesh, Mesh3d, MeshMaterial3d, Meshable, Plane3d, Query, Res, ResMut, StandardMaterial, Transform, Vec2, Vec3, With, Without};
use bevy::render::mesh::VertexAttributeValues;
use crate::components::Terrain3d;
use noise::{BasicMulti, NoiseFn, Perlin, Seedable};

pub fn terrain_added(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    added_terrains: Query<(Entity, &Terrain3d), Added<Terrain3d>>,
) {
    &added_terrains.iter().for_each(|(entity, terrain)| {
        let mut terrain_mesh = Mesh::from(
            Plane3d::default()
                .mesh()
                .size(terrain.size.x, terrain.size.z)
                .subdivisions(terrain.subdivisions.x as u32),
        );
        let noise = BasicMulti::<Perlin>::default();

        if let Some(VertexAttributeValues::Float32x3(positions, )) = terrain_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
            for pos in positions.iter_mut() {
                pos[1] = noise.get([
                    pos[0] as f64 / 300.,
                    pos[2] as f64 / 300.,
                    0.0,
                ]) as f32 * terrain.height;
            }
        }
        terrain_mesh.compute_normals();


        cmds.entity(entity).insert(Mesh3d(meshes.add(terrain_mesh)));

        //TODO: Replace with a material loaded from our assets, when a TerrainStorage is implemented
        cmds.entity(entity).insert(MeshMaterial3d(materials.add(Color::WHITE)));
        cmds.entity(entity).insert(Transform::from_xyz(0., 0., 0.));
    });
}

pub fn toggle_wireframe(
    mut cmds: Commands,
    landscapes_wireframes: Query<Entity, (With<Terrain3d>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<Terrain3d>, Without<Wireframe>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for terrain in &landscapes {
            cmds.entity(terrain).insert(Wireframe);
            debug!("Added wireframe to terrain {:?}", terrain);
        }
        for terrain in &landscapes_wireframes {
            cmds.entity(terrain).remove::<Wireframe>();
            debug!("Removed wireframe from terrain {:?}", terrain);
        }
    }
}