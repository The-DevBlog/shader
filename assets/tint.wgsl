#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// Scene texture: the rendered image.
@group(0) @binding(0)
var scene_texture: texture_2d<f32>;

// Sampler for the scene texture.
@group(0) @binding(1)
var scene_sampler: sampler;

// Tint color: the color you wish to blend into the scene.
// For example, a red tint would be vec4(1.0, 0.0, 0.0, 1.0).
// @group(0) @binding(2)
// var<uniform> tint_color: vec4<f32>;

// // Tint strength: a value in [0.0, 1.0] where 0.0 leaves the original scene unchanged,
// // and 1.0 completely replaces it with the tint color.
// @group(0) @binding(3)
// var<uniform> tint_strength: f32;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // SETTINGS
    let tint_color: vec4<f32> = vec4<f32>(1.0, 1.0, 0.0, 1.0);
    let tint_strength: f32 = 0.02; 

    // Sample the original scene color from the fullscreen texture.
    let original_color: vec4<f32> = textureSample(scene_texture, scene_sampler, in.uv);
    
    // Blend the original color with the tint color.
    // When tint_strength is 0.0, output = original_color.
    // When tint_strength is 1.0, output = tint_color.
    let output_color: vec4<f32> = mix(original_color, tint_color, tint_strength);
    
    return output_color;
}
