
#include "utils.wgsl";

struct CameraData {
    transform: mat4x4<f32>,
    position: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera_data: CameraData;


let IDENTITY_MATRIX = mat4x4<f32>(
    vec4<f32>(1.0, 0.0, 0.0, 0.0),
    vec4<f32>(0.0, 1.0, 0.0, 0.0),
    vec4<f32>(0.0, 0.0, 1.0, 0.0),
    vec4<f32>(0.0, 0.0, 0.0, 1.0)
);

@vertex
fn vs_main(
    vertex_input: InstancedVertexInput,
    instance_input: InstanceInput,
) -> TerrainVertexOutput {

    var out: TerrainVertexOutput;
    out.normal = vertex_input.normal.xyz;
    out.tex_coords = vertex_input.tex_coords;
    out.position = camera_data.transform * (IDENTITY_MATRIX * vertex_input.v_position + instance_input.i_position);
    out.original_position = vertex_input.v_position.xyz + instance_input.i_position.xyz;
    out.material_id = instance_input.instance_material_id.x;
    return out;
}

@fragment
fn fs_main(in: TerrainVertexOutput) -> @location(0) vec4<f32> {
    
    var result : vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    for (var i = 0; i < 2; i = i + 1) {
        if (i32(in.original_position.x) % 2 == 0) {
            result += vec3<f32>(0.1, 0.8, 0.2);
        } else {
            result += vec3<f32>(0.1, 0.2, 0.8);
        }
    }

    return vec4<f32>(result, 1.0);
}
