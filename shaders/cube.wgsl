

struct InstancedVertexInput {
    @location(0) v_position: vec4<f32>,
}
struct InstanceInput {
    @location(1) i_position: vec4<f32>,
}

struct TerrainVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(1) original_position: vec3<f32>,
    @location(2) normal: vec4<f32>, 
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
    out.normal = vec4<f32>(normalize(vertex_input.v_position.xyz), 1.0); 
    return out;
}

@fragment
fn fs_main(in: TerrainVertexOutput) -> @location(0) vec4<f32> {    
    let normal = normalize(in.normal);
    let light_dir = light.direction;
    
    let diffuse_intensity = max(dot(normal, light_dir), 0.0);
    let diffuse_color = light.color * diffuse_intensity;
    
    let base_color = vec4<f32>(0.1, 0.8, 0.1, 1.0); // Green terrain color
    let final_color = base_color * diffuse_color;

    return final_color;
}
