
#include "materials.wgsl";
#include "utils.wgsl";
#include "light.wgsl"; 

struct CameraData {
    transform: mat4x4<f32>,
    position: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera_data: CameraData;

@group(1) @binding(0)
var<storage, read_write> light_data: array<Light>;

@group(2) @binding(0)
var<storage, read_write> materials : array<UnTexturedMaterial>;
#include "light_utils.wgsl"; 


@vertex
fn vs_main(
    vertex_input: InstancedVertexInput,
    instance_input: InstanceInput,
) -> TerrainVertexOutput {

    let model_matrix = mat4x4<f32>(
        vec4<f32>(2.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 2.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 2.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    var out: TerrainVertexOutput;
    out.normal = vertex_input.normal.xyz;
    out.tex_coords = vertex_input.tex_coords;
    out.position = camera_data.transform * (model_matrix * vertex_input.v_position + instance_input.i_position);
    out.original_position = vertex_input.v_position.xyz + instance_input.i_position.xyz;
    out.material_id = instance_input.instance_material_id.x;
    return out;
}

@fragment
fn fs_main(in: TerrainVertexOutput) -> @location(0) vec4<f32> {
    
    var result : vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    for (var i = 0; i < 2; i = i + 1) {
        result += calc_light_terrain(in, light_data[i], i32(in.material_id));
    }

    return vec4<f32>(result, 1.0);
}
