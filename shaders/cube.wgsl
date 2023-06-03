#include "utils.wgsl";

struct CameraData {
    transform: mat4x4<f32>,
    position: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera_data: CameraData;

@vertex
fn vs_main(
    vertex_input: InstancedVertexInput,
    instance_input: InstanceInput,
) -> TerrainVertexOutput {

    let model_matrix = mat4x4<f32>(
        vec4<f32>(10.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 10.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 10.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 10.0)
    );

    var out: TerrainVertexOutput;
    out.normal = vertex_input.normal.xyz;
    out.position = camera_data.transform * (model_matrix * vertex_input.v_position);
    out.original_position = vertex_input.v_position.xyz;
    return out;
}

@fragment
fn fs_main(in: TerrainVertexOutput) -> @location(0) vec4<f32> {
    
    var result : vec3<f32> = vec3<f32>(in.position.x, 0.0, 0.0);
    return vec4<f32>(result, 1.0);
}