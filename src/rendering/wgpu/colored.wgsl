@group(0) @binding(0)
var<uniform> size: vec2<f32>;
@group(0) @binding(1)
var<uniform> scale_factor: f32;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    var physical_position = vec2(in.position.x, size.y - in.position.y) * scale_factor;
    var wgpu_position = physical_position / (size / 2.0) - 1.0;
    out.position = vec4(wgpu_position, 1.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}