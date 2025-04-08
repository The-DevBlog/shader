@group(0) @binding(0)
var<uniform> ViewProj: mat4x4<f32>;

@group(2) @binding(0)
var<uniform> material_color: vec4<f32>;

@group(2) @binding(1)
var<uniform> tint: vec4<f32>;

@group(2) @binding(2)
var<uniform> tint_strength: f32;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    // Optionally, you can pass the world position if needed:
    @location(1) world_pos: vec3<f32>,
};

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>
) -> VertexOutput {
    var out: VertexOutput;
    // Transform the vertex position to clip space.
    out.clip_position = ViewProj * vec4<f32>(position, 1.0);
    // Pass the normal unchanged.
    out.world_normal = normal;
    // Pass the world-space position, if you need it.
    out.world_pos = position;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Define a fixed directional light.
    let light_direction = normalize(vec3<f32>(0.5, 0.5, 1.0));
    let normalized_normal = normalize(in.world_normal);
    
    // Calculate diffuse lighting.
    let diffuse = max(dot(normalized_normal, light_direction), 0.0);
    let ambient = 0.1;
    let lighting = ambient + diffuse;
    
    // Compute the base lit color.
    let base_color = material_color.rgb * lighting;
    
    // --- Hue Overlay Section ---
    // Multiply the base color by the hue overlay's RGB to get a tinted version.
    let tinted_color = base_color * tint.rgb;
    
    // Use the externally provided hue_strength to blend the tint.
    // When hue_strength is 0.0, final_color equals base_color.
    // When hue_strength is 1.0, final_color is fully tinted.
    let final_color = mix(base_color, tinted_color, tint_strength);
    
    return vec4<f32>(final_color, material_color.a);
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
