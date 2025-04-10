use bevy::{
    color::palettes::{
        css::{BLUE, GREEN, LIGHT_BLUE, RED, WHITE, YELLOW},
        tailwind::BLUE_600,
    },
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_third_person_camera::{
    ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ThirdPersonCameraPlugin,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, MyExtension>>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_cube)
        .run();
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
) {
    let color = Color::srgb(0.44, 0.75, 0.44);
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(150.0, 150.0))),
        MeshMaterial3d(extended_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: color.into(),
                opaque_render_method: OpaqueRendererMethod::Auto,
                ..Default::default()
            },
            extension: MyExtension {
                tint: YELLOW.into(),
                tint_strength: 0.8,
            },
        })),
    ));

    cmds.spawn((
        Camera3d::default(),
        ThirdPersonCamera {
            zoom: Zoom::new(30.0, 100.0),
            ..default()
        },
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

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct MyExtension {
    // 0 - 99 reserved for base material
    #[uniform(100)]
    tint: LinearRgba,

    #[uniform(101)]
    tint_strength: f32,
}

impl MaterialExtension for MyExtension {
    fn fragment_shader() -> ShaderRef {
        "lighting_extended.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "lighting_extended.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "lighting_extended.wgsl".into()
    }
}

fn spawn_cube(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
    assets: Res<AssetServer>,
) {
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(25.0, 25.0, 25.0))),
        MeshMaterial3d(extended_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: BLUE_600.into(),
                // can be used in forward or deferred mode
                opaque_render_method: OpaqueRendererMethod::Auto,
                // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
                // in forward mode, the output can also be modified after lighting is applied.
                // see the fragment shader `extended_material.wgsl` for more info.
                // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
                // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
                ..Default::default()
            },
            extension: MyExtension {
                tint: YELLOW.into(),
                tint_strength: 0.8,
            },
        })),
        // SceneRoot(assets.load("tank_gen_2.gltf#Scene0")),
        ThirdPersonCameraTarget,
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
    ));
}
