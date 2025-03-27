use bevy::{
    color::palettes::css::RED,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_third_person_camera::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, OutlineMaterial>>::default(),
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Startup, (setup, spawn_cube))
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn((
        Camera3d::default(),
        ThirdPersonCamera::default(),
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    cmds.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            150.0f32.to_radians(),
            -40.0f32.to_radians(),
            0.0,
        )),
    ));
}

fn spawn_cube(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, OutlineMaterial>>>,
) {
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 10.0))),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: RED.into(),
                ..default()
            },
            extension: OutlineMaterial { quantize_steps: 3 },
        })),
        ThirdPersonCameraTarget,
    ));
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct OutlineMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    pub quantize_steps: u32,
}

impl MaterialExtension for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shader.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shader.wgsl".into()
    }
}
