use bevy::{
    color::palettes::css::{GREEN, RED},
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_third_person_camera::{
    ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ThirdPersonCameraPlugin,
            // MaterialPlugin::<ExtendedMaterial<StandardMaterial, CustomMaterial>>::default(),
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        // .add_systems(Startup, (setup, spawn))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_cube)
        // .add_systems(Update, update_frame_system)
        .run();
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    let color = Color::srgb(0.44, 0.75, 0.44);
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(custom_materials.add(CustomMaterial {
            color: color.into(),
        })),
        // MeshMaterial3d(materials.add(Color::srgb(0.44, 0.75, 0.44))),
        Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)),
    ));

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

// ----------------- EXTENDED MATERIAL SHADER ------------------
fn spawn(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CustomMaterial>>>,
) {
    // sphere
    // cmds.spawn((
    //     // Mesh3d(meshes.add(Sphere::new(1.0))),
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(ExtendedMaterial {
    //         base: StandardMaterial {
    //             base_color: RED.into(),
    //             // can be used in forward or deferred mode
    //             opaque_render_method: OpaqueRendererMethod::Auto,
    //             // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
    //             // in forward mode, the output can also be modified after lighting is applied.
    //             // see the fragment shader `extended_material.wgsl` for more info.
    //             // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
    //             // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
    //             ..Default::default()
    //         },
    //         extension: CustomMaterial {
    //             color: LinearRgba::BLUE,
    //         },
    //     })),
    //     Transform::from_xyz(0.0, 1.0, 0.0),
    // ));
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct CustomMaterial {
    // This uniform will be sent to the shader as "material_color"
    #[uniform(0)]
    color: LinearRgba,
}

// #[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
// struct MyExtension {
//     // 0 - 99 reserved for base material
//     #[uniform(100)]
//     quantize_steps: u32,
// }

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "lighting.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "lighting.wgsl".into()
    }
    // fn fragment_shader() -> ShaderRef {
    //     "extended_material_shader.wgsl".into()
    // }

    // fn deferred_fragment_shader() -> ShaderRef {
    //     "extended_material_shader.wgsl".into()
    // }
}

// ----------------- CUSTOM MATERIAL SHADER ------------------
fn spawn_cube(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    // mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CustomMaterial>>>,
) {
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(custom_materials.add(CustomMaterial {
            // frame: 1,
            color: LinearRgba::BLUE,
        })),
        // MeshMaterial3d(materials.add(ExtendedMaterial {
        //     base: StandardMaterial {
        //         base_color: RED.into(),
        //         // can be used in forward or deferred mode
        //         opaque_render_method: OpaqueRendererMethod::Auto,
        //         // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
        //         // in forward mode, the output can also be modified after lighting is applied.
        //         // see the fragment shader `extended_material.wgsl` for more info.
        //         // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
        //         // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
        //         ..Default::default()
        //     },
        //     extension: CustomMaterial {
        //         color: LinearRgba::BLUE,
        //     },
        // })),
        ThirdPersonCameraTarget,
        // Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
    ));
}

// fn update_frame_system(mut custom_materials: ResMut<Assets<CustomMaterial>>) {
//     // Iterate over all CustomMaterial assets.
//     for (_handle, material) in custom_materials.iter_mut() {
//         // Increment the frame value.
//         // wrapping_add is used to safely handle overflow.
//         material.frame = material.frame.wrapping_add(1);
//     }
// }

// #[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// struct CustomMaterial {
//     // #[uniform(0)]
//     // color: LinearRgba,
//     #[uniform(0)]
//     frame: u32,
// }

// impl Material for CustomMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "custom_material_shader.wgsl".into()
//     }
// }
