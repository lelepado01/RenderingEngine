

struct InstancedVertexInput {
    @location(0) v_position: vec4<f32>,
}
struct InstanceInput {
    @location(1) i_position: vec4<f32>,
}

struct TerrainVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) original_position: vec3<f32>,
};

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
    out.position = camera_data.transform * (IDENTITY_MATRIX * vertex_input.v_position + instance_input.i_position);
    out.original_position = vertex_input.v_position.xyz + instance_input.i_position.xyz;
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
