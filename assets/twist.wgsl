#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// Creates a twist effect in the center of the screen

// The rendered scene texture.
@group(0) @binding(0)
var scene_texture: texture_2d<f32>;

// Sampler for the scene texture.
@group(0) @binding(1)
var scene_sampler: sampler;

// Uniform settings for the twist effect.
// struct MoebiusSettings {
//   // Twist amount, in radians—defines the maximum rotation at the center.
//   twist_amount: f32,
//   // Radius (in UV space, e.g. 0.0 to 1.0) within which the twist effect applies.
//   radius: f32,
//   // Center of the twist effect (typically (0.5, 0.5) for screen center).
//   center: vec2<f32>,
// }
// @group(0) @binding(2)
// var<uniform> settings: MoebiusSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let twist_amount: f32 = 1.0; // Maximum twist angle in radians.
    let radius: f32 = 0.1; // Radius of the twist effect in UV space.
    let center: vec2<f32> = vec2<f32>(0.5, 0.5); // Center of the twist effect in UV space.

    let uv: vec2<f32> = in.uv;

    // Compute the offset from the center.
    let offset: vec2<f32> = uv - center;
    let dist: f32 = length(offset);

    // Determine the twist angle. Within the specified radius, the twist decreases linearly
    // from full twist_amount at the center (dist = 0) to zero at the boundary (dist = radius).
    var angle: f32 = 0.0;
    if (dist < radius) {
    angle = twist_amount * (1.0 - dist / radius);
    }

    // Create a 2D rotation matrix.
    let cos_angle: f32 = cos(angle);
    let sin_angle: f32 = sin(angle);
    let rot: mat2x2<f32> = mat2x2<f32>(
    vec2<f32>(cos_angle, -sin_angle),
    vec2<f32>(sin_angle,  cos_angle)
    );

    // Apply rotation to the offset.
    let twisted_offset: vec2<f32> = rot * offset;

    // Compute the new UV coordinates.
    let twisted_uv: vec2<f32> = center + twisted_offset;

    // Sample the scene texture using the twisted UV.
    let color: vec4<f32> = textureSample(scene_texture, scene_sampler, twisted_uv);
    return color;
}
