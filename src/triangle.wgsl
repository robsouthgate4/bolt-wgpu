// triangle.wgsl

@vertex
fn vs_main(@builtin(vertex_index) VertexIndex: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>( 0.0,  0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>( 0.5, -0.5)
    );

    let pos = positions[VertexIndex];
    return vec4<f32>(pos, 0.0, 1.0);
}

struct Uniforms {
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(sin(uniforms.time)*0.5+0.5, 0.0, 0.4, 1.0);
}