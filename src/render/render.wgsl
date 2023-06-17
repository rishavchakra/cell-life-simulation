struct SimulationParams {
    width: u32,
    height: u32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv_coord: vec2<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) uv_coord: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = position;
    out.uv_coord = uv_coord;
    return out;
}

@fragment
fn fs_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4<f32>(0.2, 0.3, 0.4, 1.0);
}
