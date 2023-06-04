
#include "materials.wgsl";
#include "utils.wgsl";
#include "light.wgsl"; 
#include "camera.wgsl"; 

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

    let size = instance_input.instance_position.w; 
    let model_matrix = mat4x4<f32>(
        vec4<f32>(size, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, size, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, size, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    let instance_pos = vec4<f32>(instance_input.instance_position.xyz, 1.0);

    var out: TerrainVertexOutput;
    out.normal = vertex_input.normal.xyz;
    out.position = camera_data.transform * (model_matrix * vertex_input.v_position + instance_pos);
    out.original_position = vertex_input.v_position.xyz + instance_pos.xyz;
    out.material_id = instance_input.instance_material_id.x; 

    return out;
}

@fragment
fn fs_main(in: TerrainVertexOutput) -> @location(0) vec4<f32> {

    // if (in.material_id == 0.0) {
    //     discard;
    // }
    
    var result : vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    for (var i = 0; i < 2; i = i + 1) {
        result += calc_light_terrain(in, light_data[i], i32(in.material_id));
    }

    return vec4<f32>(result, 1.0);
}
