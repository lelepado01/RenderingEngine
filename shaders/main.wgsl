
#include "consts.wgsl"; 

struct InstancedVertexInput {
    @location(0) v_position: vec4<f32>,
}
struct InstanceInput {
    @location(1) i_position: vec4<f32>,
    @location(2) i_size: vec4<f32>,
}

struct TerrainVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) original_position: vec3<f32>,
    @location(2) normal: vec4<f32>, 
    @location(3) idx : i32, 
};

struct CameraData {
    transform: mat4x4<f32>,
    position: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera_data: CameraData;

struct Light {
    direction: vec4<f32>,
    color: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> light: Light;

struct Material {
    diffuse_color: vec4<f32>,
    specular_color: vec3<f32>,
    shininess: f32,
    metallic: f32,
    roughness: f32,
};

@group(2) @binding(0)
var<storage, read> materials: array<Material>;

@vertex
fn vs_main(
    vertex_input: InstancedVertexInput,
    instance_input: InstanceInput,
) -> TerrainVertexOutput {

    var out: TerrainVertexOutput;
    out.position = camera_data.transform * (IDENTITY_MATRIX * vertex_input.v_position * instance_input.i_size + instance_input.i_position);
    out.original_position = vertex_input.v_position.xyz + instance_input.i_position.xyz;
    out.normal = vec4<f32>(normalize(vertex_input.v_position.xyz), 1.0); 
    out.idx = i32(instance_input.i_position.x) % 8; 
    return out;
}

@fragment
fn fs_main(in: TerrainVertexOutput) -> @location(0) vec4<f32> {    

    let material = materials[in.idx];

    let normal = normalize(in.normal.xyz);
    let light_dir = normalize(light.direction.xyz); 
    let view_dir = normalize(camera_data.position.xyz - in.position.xyz);
    let reflect_dir = reflect(-light_dir, normal);

    // Ambient light
    let ambient_strength = 0.2;
    let ambient = material.diffuse_color.rgb * ambient_strength;

    // Diffuse lighting
    let diffuse_intensity = max(dot(normal, light_dir), 0.0);
    let diffuse = material.diffuse_color.rgb * diffuse_intensity;

    // Specular lighting (Phong)
    let specular_strength = material.shininess; // Defined per material
    let shininess = material.shininess;
    let specular_intensity = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
    let specular = material.specular_color.rgb * specular_strength * specular_intensity;

    // Final color calculation
    let final_color = ambient + diffuse + specular;

    return vec4<f32>(final_color, 1.0);
}