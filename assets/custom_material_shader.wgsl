// #import bevy_pbr::forward_io::VertexOutput;

// const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 0.5);

// @group(2) @binding(0) var<uniform> material_color: vec4<f32>;

// @fragment 
// fn fragment(
//     mesh: VertexOutput,
// ) -> @location(0) vec4<f32> {
//     return material_color * COLOR_MULTIPLIER;
// }

@binding(0) @group(2) var<uniform> frame : u32;
@vertex
fn vertex(@builtin(vertex_index) vertex_index : u32) -> @builtin(position) vec4f {
  const pos = array(
    vec2( 0.0,  0.5),
    vec2(-0.5, -0.5),
    vec2( 0.5, -0.5)
  );

  return vec4f(pos[vertex_index], 0, 1);
}
@fragment
fn fragment() -> @location(0) vec4f {
  return vec4(1, sin(f32(frame) / 128), 0, 1);
}
