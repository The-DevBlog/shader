// @group(0) @binding(0)
// var<uniform> ViewProj: mat4x4<f32>;

// @group(2) @binding(0)
// var<uniform> material_color: vec4<f32>;

// @group(2) @binding(100)
// var<uniform> tint: vec4<f32>;

// @group(2) @binding(101)
// var<uniform> tint_strength: f32;

// --- Vertex Input/Output Structures ---
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    // UV coordinates from the vertex shader.
    @location(1) uv: vec2<f32>,
};

// --- Uniforms ---
// Camera view-projection matrix. (Provided in group 0, binding 0)
@group(0) @binding(0)
var<uniform> ViewProj: mat4x4<f32>;

// Custom material extension uniforms in group 2.
// We’re using custom bindings 101 and 102 for our tint parameters.
@group(2) @binding(101)
var<uniform> tint: vec4<f32>;

@group(2) @binding(102)
var<uniform> tint_strength: f32;

// Declare the base texture and its sampler that come from your StandardMaterial’s texture.
// These binding numbers must match the pipeline layout or material extension setup.
@group(2) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_color_sampler: sampler;

// Optionally, if you want a fallback for objects that have no texture,
// you can also use a uniform color. (This is not used if the texture is available.)
@group(2) @binding(100)
var<uniform> base_color: vec4<f32>;

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    // Transform the vertex position to clip space.
    out.clip_position = ViewProj * vec4<f32>(position, 1.0);
    // Pass the normal unchanged.
    out.world_normal = normal;
    out.uv = uv;
    // Pass the world-space position, if you need it.
    // out.world_pos = position;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // --- Lighting Computation ---
    // Define a fixed directional light.
    let light_direction = normalize(vec3<f32>(0.5, 0.5, 1.0));
    let normal = normalize(in.world_normal);
    let diffuse = max(dot(normal, light_direction), 0.0);
    let ambient = 0.1;
    let lighting = ambient + diffuse;

    // --- Base Color from Texture ---
    // Sample the base texture using the UV coordinates.
    // You can also decide to blend the uniform base_color if no texture is provided.
    let tex_color = textureSample(base_color_texture, base_color_sampler, in.uv);

    // Multiply the texture color by the lighting to get a lit base color.
    let lit_color = tex_color.rgb * lighting;

    // --- Tint Application ---
    // Multiply the lit color by the tint color to produce a tinted version.
    let tinted_color = lit_color * tint.rgb;
    // Linearly interpolate between the lit base color and the tinted variant,
    // using tint_strength (0.0 = no tint, 1.0 = full tint).
    let final_color = mix(lit_color, tinted_color, tint_strength);

    return vec4<f32>(final_color, tex_color.a);
}

// @fragment
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
//     // Define a fixed directional light.
//     let light_direction = normalize(vec3<f32>(0.5, 0.5, 1.0));
    
//     // Normalize the normal passed from the vertex shader.
//     let normalized_normal = normalize(in.world_normal);
    
//     // Calculate the diffuse lighting using the dot product.
//     let diffuse = max(dot(normalized_normal, light_direction), 0.0);
    
//     // Add a small ambient light so that surfaces aren’t pitch black.
//     let ambient = 0.1;
    
//     // Combine ambient and diffuse lighting.
//     let lighting = ambient + diffuse;
    
//     // Compute the final color by modulating the material's base color.
//     let final_color = material_color.rgb * lighting;
    
//     // Return the final color with the original alpha component.
//     return vec4<f32>(final_color, material_color.a);
// }
