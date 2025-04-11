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
        .init_resource::<MyAssets>()
        .init_resource::<LoadShaders>()
        .add_plugins((
            DefaultPlugins,
            ThirdPersonCameraPlugin,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, MyExtension>>::default(),
        ))
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_cube)
        .add_systems(Update, customize_scene_materials.run_if(load_shaders))
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

#[derive(Resource)]
struct LoadShaders(bool);

impl Default for LoadShaders {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Resource, Default)]
struct MyAssets {
    tank: Handle<Scene>,
    tank_gltf: Handle<Gltf>,
}

fn load_shaders(load_shaders: Res<LoadShaders>) -> bool {
    load_shaders.0
}

fn load_assets(assets: Res<AssetServer>, mut my_assets: ResMut<MyAssets>) {
    my_assets.tank = assets.load("tank_gen_2.gltf#Scene0");
    my_assets.tank_gltf = assets.load("tank_gen_2.gltf");
}

fn spawn_cube(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
    gltf: Res<Assets<Gltf>>,
    assets: Res<AssetServer>,
    my_assets: Res<MyAssets>,
) {
    cmds.spawn((
        // Mesh3d(meshes.add(Cuboid::new(25.0, 25.0, 25.0))),
        // MeshMaterial3d(extended_materials.add(ExtendedMaterial {
        //     base: StandardMaterial {
        //         base_color: BLUE_600.into(),
        //         // can be used in forward or deferred mode
        //         opaque_render_method: OpaqueRendererMethod::Auto,
        //         // in deferred mode, only the PbrInput can be modified (uvs, color and other material properties),
        //         // in forward mode, the output can also be modified after lighting is applied.
        //         // see the fragment shader `extended_material.wgsl` for more info.
        //         // Note: to run in deferred mode, you must also add a `DeferredPrepass` component to the camera and either
        //         // change the above to `OpaqueRendererMethod::Deferred` or add the `DefaultOpaqueRendererMethod` resource.
        //         ..Default::default()
        //     },
        //     extension: MyExtension {
        //         tint: YELLOW.into(),
        //         tint_strength: 0.8,
        //     },
        // })),
        SceneRoot(my_assets.tank.clone()),
        ThirdPersonCameraTarget,
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
    ));
}

fn customize_scene_materials(
    gltf_assets: Res<Assets<Gltf>>,
    my_assets: Res<MyAssets>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut load_shaders: ResMut<LoadShaders>,
    mut q_scenes: Query<(Entity), With<SceneRoot>>,
    mut cmds: Commands,
) {
    for ent in q_scenes.iter_mut() {
        cmds.entity(ent).remove::<StandardMaterial>();
    }

    // let Some(gltf_asset) = gltf_assets.get(&my_assets.tank_gltf) else {
    //     return;
    // };

    // load_shaders.0 = false;

    // for material_handle in &gltf_asset.materials {
    //     let Some(material) = standard_materials.get_mut(material_handle) else {
    //         continue;
    //     };

    //     material.base_color = Color::srgb(0.0, 0.0, 1.0).into(); // Blue
    // }
}
