#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// Screen texture: the rendered scene.
@group(0) @binding(0)
var scene_texture: texture_2d<f32>;

// Sampler for the screen texture.
@group(0) @binding(1)
var scene_sampler: sampler;

// Hardcoded resolution; in practice, you might want this to be a uniform.
const resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);

// ---------- Helper Functions ----------

// Quantize color to 'levels' discrete steps per channel.
fn quantize_color(c: vec3<f32>, levels: u32) -> vec3<f32> {
    let levels_f: f32 = f32(levels);
    return floor(c * levels_f + 0.5) / levels_f;
}

// Boost the saturation: mix the color with its grayscale (luma) value.
fn saturate_color(c: vec3<f32>, boost: f32) -> vec3<f32> {
    let l: f32 = dot(c, vec3<f32>(0.299, 0.587, 0.114));
    return mix(vec3<f32>(l), c, boost);
}

// Compute a Sobel edge mask based on luminance gradients.
// The texel size is scaled by `outline_width` to increase the effective sampling area.
fn sobel_edge_mask(uv: vec2<f32>, outline_width: f32) -> f32 {
    // Compute texel size adjusted by outline_width.
    let ts: vec2<f32> = (vec2<f32>(outline_width)) / resolution;
    
    // Sample a 3x3 neighborhood and compute luma for each sample.
    let i00: f32 = dot(textureSample(scene_texture, scene_sampler, uv + vec2<f32>(-ts.x, -ts.y)).rgb, vec3<f32>(0.299, 0.587, 0.114));
    let i01: f32 = dot(textureSample(scene_texture, scene_sampler, uv + vec2<f32>(0.0,    -ts.y)).rgb, vec3<f32>(0.299, 0.587, 0.114));
    let i02: f32 = dot(textureSample(scene_texture, scene_sampler, uv + vec2<f32>( ts.x,  -ts.y)).rgb, vec3<f32>(0.299, 0.587, 0.114));
    
    let i10: f32 = dot(textureSample(scene_texture, scene_sampler, uv + vec2<f32>(-ts.x,  0.0)).rgb, vec3<f32>(0.299, 0.587, 0.114));
    let i11: f32 = dot(textureSample(scene_texture, scene_sampler, uv).rgb, vec3<f32>(0.299, 0.587, 0.114));
    let i12: f32 = dot(textureSample(scene_texture, scene_sampler, uv + vec2<f32>( ts.x,  0.0)).rgb, vec3<f32>(0.299, 0.587, 0.114));
    
    let i20: f32 = dot(textureSample(scene_texture, scene_sampler, uv + vec2<f32>(-ts.x,  ts.y)).rgb, vec3<f32>(0.299, 0.587, 0.114));
    let i21: f32 = dot(textureSample(scene_texture, scene_sampler, uv + vec2<f32>( 0.0,    ts.y)).rgb, vec3<f32>(0.299, 0.587, 0.114));
    let i22: f32 = dot(textureSample(scene_texture, scene_sampler, uv + vec2<f32>( ts.x,   ts.y)).rgb, vec3<f32>(0.299, 0.587, 0.114));
    
    // Compute Sobel horizontal (gx) and vertical (gy) gradients.
    let gx: f32 = -1.0 * i00 + 0.0 * i01 + 1.0 * i02 +
                    -2.0 * i10 + 0.0 * i11 + 2.0 * i12 +
                    -1.0 * i20 + 0.0 * i21 + 1.0 * i22;
                    
    let gy: f32 = -1.0 * i00 - 2.0 * i01 - 1.0 * i02 +
                     0.0 * i10 +  0.0 * i11 +  0.0 * i12 +
                     1.0 * i20 +  2.0 * i21 +  1.0 * i22;
    
    return sqrt(gx * gx + gy * gy);
}

// Compute an outline mask: if the edge strength is above the given threshold,
// return the edge value; otherwise, return 0.
fn outline_mask(uv: vec2<f32>, threshold: f32, outline_width: f32) -> f32 {
    let edge_val: f32 = sobel_edge_mask(uv, outline_width);
    return select(0.0, edge_val, edge_val > threshold);
}

// ---------- Main Fragment Shader ----------

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // SETTINGS:
    let quant_levels: u32 = 32u;           // Number of quantization levels per channel.
    let blend_factor: f32 = 0.5;           // Blend factor: 0.0 = original, 1.0 = stylized.
    let saturation_boost: f32 = 1.0;         // Saturation boost: 1.0 = no change, >1.0 = increased saturation.
    let outline_threshold: f32 = 0.05;       // Edge detection threshold.
    let outline_strength: f32 = 10.0;        // Strength to mix in the outline color.
    let outline_color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0); // Outline color (black).
    
    // New parameter: outline width. Increase this value to broaden the outline.
    let outline_width: f32 = 3.0;

    let uv: vec2<f32> = in.uv;
    
    // Sample the original scene color.
    let original_color: vec3<f32> = textureSample(scene_texture, scene_sampler, uv).rgb;
    
    // Create a stylized (posterized) version:
    let quantized: vec3<f32> = quantize_color(original_color, quant_levels);
    let sat_quantized: vec3<f32> = saturate_color(quantized, saturation_boost);
    
    // Blend original and stylized versions.
    let stylized: vec3<f32> = mix(original_color, sat_quantized, blend_factor);
    
    // Compute the outline mask using the specified outline_width.
    let mask: f32 = outline_mask(uv, outline_threshold, outline_width);
    
    // Mix in the outline color (black) based on the mask, scaled by outline_strength.
    let color_with_outline: vec3<f32> = mix(stylized, outline_color.rgb, mask * outline_strength);
    
    return vec4<f32>(color_with_outline, 1.0);
}
