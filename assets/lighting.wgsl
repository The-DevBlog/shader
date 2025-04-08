@group(0) @binding(0)
var<uniform> ViewProj: mat4x4<f32>;

@group(2) @binding(0)
var<uniform> material_color: vec4<f32>;

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
    
    // Normalize the normal passed from the vertex shader.
    let normalized_normal = normalize(in.world_normal);
    
    // Calculate the diffuse lighting using the dot product.
    let diffuse = max(dot(normalized_normal, light_direction), 0.0);
    
    // Add a small ambient light so that surfaces aren’t pitch black.
    let ambient = 0.1;
    
    // Combine ambient and diffuse lighting.
    let lighting = ambient + diffuse;
    
    // Compute the base final color by modulating the material's color.
    let base_color = material_color.rgb * lighting;
    
    // --- Hue Shift Section ---
    // Set a hue shift amount in radians (adjust as desired).
    let hue_shift: f32 = 0.2;
    let cosA = cos(hue_shift);
    let sinA = sin(hue_shift);
    
    // Create a hue rotation matrix.
    // This common matrix rotates the hue of an RGB color.
    let hue_rotation: mat3x3<f32> = mat3x3<f32>(
        vec3<f32>(0.213 + 0.787 * cosA - 0.213 * sinA,
                   0.715 - 0.715 * cosA - 0.715 * sinA,
                   0.072 - 0.072 * cosA + 0.928 * sinA),
                   
        vec3<f32>(0.213 - 0.213 * cosA + 0.143 * sinA,
                   0.715 + 0.285 * cosA + 0.140 * sinA,
                   0.072 - 0.072 * cosA - 0.283 * sinA),
                   
        vec3<f32>(0.213 - 0.213 * cosA - 0.787 * sinA,
                   0.715 - 0.715 * cosA + 0.715 * sinA,
                   0.072 + 0.928 * cosA + 0.072 * sinA)
    );
    
    // Apply the hue rotation to our base color.
    let final_color = hue_rotation * base_color;
    
    // Return the final color with the original alpha.
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
