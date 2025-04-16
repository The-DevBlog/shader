use bevy::{
    color::palettes::{
        css::{BLUE, GREEN, LIGHT_BLUE, RED, WHITE, YELLOW},
        tailwind::BLUE_600,
    },
    core_pipeline::{
        fxaa::{Fxaa, Sensitivity},
        prepass::{DepthPrepass, NormalPrepass},
    },
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    scene::SceneInstanceReady,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::{
    ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom,
};

mod stylized;
mod tint;

use stylized::{StylizedShaderPlugin, StylizedShaderSettings};
use tint::{TintShaderPlugin, TintShaderSettings};

const TINT_STRENGTH: f32 = 0.8;
fn main() {
    App::new()
        .init_resource::<MyAssets>()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            // TintShaderPlugin,
            // DefaultPlugins,
            WorldInspectorPlugin::new(),
            StylizedShaderPlugin,
            ThirdPersonCameraPlugin,
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, MyExtension>>::default(),
        ))
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_tank)
        .add_systems(Update, move_cube)
        // .add_observer(customize_scene_materials)
        .run();
}

#[derive(Component)]
struct Tank;

#[derive(Resource, Default)]
struct MyAssets {
    tank: Handle<Scene>,
}

fn move_cube(
    mut q: Query<&mut Transform, With<Tank>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut transform) = q.get_single_mut() else {
        return;
    };

    let time = time.delta_secs();
    let speed = 10.0;

    if input.pressed(KeyCode::KeyA) {
        transform.translation.x -= time * speed;
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation.x += time * speed;
    }
    if input.pressed(KeyCode::KeyW) {
        transform.translation.z += time * speed;
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation.z -= time * speed;
    }
    if input.pressed(KeyCode::KeyQ) {
        transform.rotate_local_y(time * speed);
    }
    if input.pressed(KeyCode::KeyE) {
        transform.rotate_local_y(time * -speed);
    }
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut extended_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground
    let color = Color::srgb(0.44, 0.75, 0.44);
    cmds.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(150.0, 150.0))),
        MeshMaterial3d(standard_materials.add(StandardMaterial::from_color(color))),
        // MeshMaterial3d(extended_materials.add(ExtendedMaterial {
        //     base: StandardMaterial {
        //         base_color: color.into(),
        //         opaque_render_method: OpaqueRendererMethod::Auto,
        //         ..Default::default()
        //     },
        //     extension: MyExtension {
        //         base_color: color.into(),
        //         tint: YELLOW.into(),
        //         tint_strength: TINT_STRENGTH,
        //     },
        // })),
    ));

    // camera
    cmds.spawn((
        Camera3d::default(),
        // TintShaderSettings::default(),
        StylizedShaderSettings::default(),
        DepthPrepass,
        NormalPrepass,
        Msaa::Off,
        Fxaa {
            enabled: true,
            edge_threshold: Sensitivity::Ultra,
            edge_threshold_min: Sensitivity::Ultra,
        },
        ThirdPersonCamera {
            zoom: Zoom::new(30.0, 200.0),
            ..default()
        },
        Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("Camera"),
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
    base_color: LinearRgba,

    #[uniform(101)]
    tint: LinearRgba,

    #[uniform(102)]
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

fn load_assets(assets: Res<AssetServer>, mut my_assets: ResMut<MyAssets>) {
    my_assets.tank = assets.load(GltfAssetLabel::Scene(0).from_asset("tank_gen_2.gltf"));
}

fn spawn_tank(mut cmds: Commands, my_assets: Res<MyAssets>) {
    cmds.spawn((
        SceneRoot(my_assets.tank.clone()),
        ThirdPersonCameraTarget,
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
        Tank,
    ));
}

// fn customize_scene_materials(
//     trigger: Trigger<SceneInstanceReady>,
//     mut extended_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, MyExtension>>>,
//     standard_materials: Res<Assets<StandardMaterial>>,
//     mut cmds: Commands,
//     q_children: Query<&Children>,
//     q_mesh_material: Query<(Entity, &MeshMaterial3d<StandardMaterial>)>,
// ) {
//     // Traverse the spawned SceneRoot's descendants.
//     for entity in q_children.iter_descendants(trigger.entity()) {
//         // Try to get a MeshMaterial3d<StandardMaterial> component on this entity.
//         if let Ok((ent, mesh_mat)) = q_mesh_material.get(entity) {
//             println!("Changing");
//             // Use the handle from the MeshMaterial3d to fetch the StandardMaterial.
//             if let Some(std_mat) = standard_materials.get(mesh_mat.id()) {
//                 // Optionally, clone and modify the StandardMaterial.
//                 let modified_std = std_mat.clone();
//                 // (For example, you could change the base color here before wrapping.)
//                 // modified_std.base_color = Color::rgb(0.0, 0.0, 1.0).into();
//                 let base_color = modified_std.base_color;
//                 // Now create an ExtendedMaterial that wraps the StandardMaterial.
//                 let new_extended_handle = extended_materials.add(ExtendedMaterial {
//                     base: modified_std,
//                     extension: MyExtension {
//                         base_color: base_color.into(),
//                         tint: YELLOW.into(),          // Your desired tint color.
//                         tint_strength: TINT_STRENGTH, // How strongly to apply the tint.
//                     },
//                 });

//                 // Replace the material component on this entity:
//                 // Option 1: Remove the old material component and insert the new one.
//                 cmds.entity(ent)
//                     .remove::<MeshMaterial3d<StandardMaterial>>() // Remove the original.
//                     .insert(MeshMaterial3d(new_extended_handle)); // Insert the new, extended one.
//             }
//         }
//     }
// }
// }
// }
