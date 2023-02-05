struct TimeUniform {
    time: f32,
}

@group(0) @binding(0)
var<uniform> time_uniform: TimeUniform;

struct Vertex {
    @location(0) position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv_position: vec2<f32>,
}

@vertex
fn vs_main(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(vertex.position, 0.0, 1.0);
    out.uv_position = (vertex.position + vec2<f32>(1.0, 1.0)) / 2.0;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec3<f32> = vec3<f32>(1.0);
    return vec4<f32>(color, 1.0);
}
