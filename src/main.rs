use bevy::{
    color::palettes::css::{GREEN, RED},
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_rts_camera::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RtsCameraPlugin,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, MyExtension>>::default(),
            // MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, (setup, spawn))
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn((
        Camera3d::default(),
        RtsCamera::default(),
        RtsCameraControls::default(),
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

// ----------------- EXTENDED MATERIAL SHADER ------------------
fn spawn(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
) {
    // sphere
    cmds.spawn((
        // Mesh3d(meshes.add(Sphere::new(1.0))),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: RED.into(),
                // can be used in forward or deferred mode
                opaque_render_method: OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                ..Default::default()
            },
            extension: MyExtension { quantize_steps: 3 },
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct MyExtension {
    // 0 - 99 reserved for base material
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        "extended_material_shader.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "extended_material_shader.wgsl".into()
    }
}

// ----------------- CUSTOM MATERIAL SHADER ------------------
// fn spawn_cube(
//     mut cmds: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut custom_materials: ResMut<Assets<CustomMaterial>>,
// ) {
//     cmds.spawn((
//         Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
//         // MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
//         MeshMaterial3d(custom_materials.add(CustomMaterial {
//             color: LinearRgba::BLUE,
//         })),
//     ));
// }

// #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// struct CustomMaterial {
//     #[uniform(0)]
//     color: LinearRgba,
// }

// impl Material for CustomMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "custom_material_shader.wgsl".into()
//     }
// }
