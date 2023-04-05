
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



// @group(2) @binding(0)
// var t_diffuse: texture_2d<f32>;
// @group(2)@binding(1)
// var s_diffuse: sampler;
// @group(2)@binding(2)
// var t_normal: texture_2d<f32>;
// @group(2) @binding(3)
// var s_normal: sampler;

@vertex
fn vs_main(
    vertex_input: VertexInput,
    instance_input: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.normal = vertex_input.normal.xyz;
    out.tex_coords = vertex_input.tex_coords;
    out.position = camera_data.transform * (vertex_input.v_position + instance_input.i_position);
    out.original_position = vertex_input.v_position.xyz + instance_input.i_position.xyz;
    out.instance_material_id = instance_input.material_id;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    
    var result : vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    for (var i = 0; i < 2; i = i + 1) {
        result += calc_light(in, light_data[i], i32(in.instance_material_id));
    }

    return vec4<f32>(result, 1.0);
}
