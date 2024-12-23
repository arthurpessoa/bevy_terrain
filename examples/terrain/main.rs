use bevy::app::PluginGroupBuilder;
use bevy::color::palettes::css::TOMATO;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::pbr::light_consts::lux::OVERCAST_DAY;
use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;
use bevy::render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_terrain::{Terrain3d, Terrain3dPlugin};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            defaults(),
            FrameTimeDiagnosticsPlugin,
            PanOrbitCameraPlugin,
            WireframePlugin,
            Terrain3dPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_fps_text)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    //Light
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 2., 0.),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));

    //Camera
    commands.spawn((
        Camera3d::default(),
        PanOrbitCamera {
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Middle,
            ..default()
        },
        Transform::from_xyz(0., 20., 75.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    // circular base
    commands.spawn((
        Terrain3d::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10., 10., 10.))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 0, 255))),
        Transform::from_xyz(15., 0., 20.),
    ));


    commands.spawn((
        Text::new("FPS: "),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor::from(TOMATO),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        FpsText,
    ));
}

fn defaults() -> PluginGroupBuilder {
    DefaultPlugins
        .set(AssetPlugin {
            ..Default::default()
        })
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            ..default()
        })
}

#[derive(Component)]
struct FpsText;


fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.0 = format!("FPS: {value:.2}");
            }
        }
    }
}