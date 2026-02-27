// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

struct ColorUniform {
    value: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> u_color: ColorUniform;

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return u_color.value;
}

