
#include "materials.wgsl";
#include "utils.wgsl";
#include "light.wgsl"; 

struct InstanceInput {
    @location(3) instance_material_id: u32,
}

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

let XSIZE : u32 = 250u;
let YSIZE : u32 = 50u;
let ZSIZE : u32 = 250u;

fn index_to_xyz(index: u32) -> vec4<f32> {
    let x = index % XSIZE;
    let y = (index / XSIZE) % YSIZE;
    let z = index / (XSIZE * YSIZE);
    return vec4<f32>(f32(x), f32(y), f32(z), 0.0);
}

fn get_byte_from_u32(value: u32, byte_index: u32) -> u32 {
    return (value >> (byte_index * 8u)) & 0xFFu;
}

@vertex
fn vs_main(
    vertex_input: InstancedVertexInput,
    instance_input: InstanceInput,
    @builtin(instance_index) instance_index: u32,
) -> TerrainVertexOutput {

    let model_matrix = mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    let instance_pos : vec4<f32> = index_to_xyz(instance_index);

    var out: TerrainVertexOutput;
    out.normal = vertex_input.normal.xyz;
    out.position = camera_data.transform * (model_matrix * vertex_input.v_position + instance_pos);
    out.original_position = vertex_input.v_position.xyz + instance_pos.xyz;
    out.material_id = f32(get_byte_from_u32(instance_input.instance_material_id, 0u));

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
