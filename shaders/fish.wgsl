
#include "light_data.wgsl"; 
#include "materials.wgsl";
#include "utils.wgsl";

struct CameraData {
    transform: mat4x4<f32>,
    position: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera_data: CameraData;

@group(1) @binding(0)
var<storage, read_write> light_data: array<Light>;

@group(2) @binding(0)
var<storage, read_write> cube_material : array<UnTexturedMaterial>;

@vertex
fn vs_main(
    vertex_input: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.normal = vertex_input.normal.xyz;
    out.tex_coords = vertex_input.tex_coords;
    out.position = camera_data.transform * vertex_input.v_position;
    out.original_position = vertex_input.v_position.xyz;
    return out;
}

fn calc_light(in: VertexOutput, light : Light) -> vec3<f32> {

    var material_index : i32 = 0;

    let ambient_strength = 0.2;
    let ambient : vec3<f32> = light.ambient * ambient_strength * cube_material[material_index].ambient;
  	
    // diffuse 
    let norm : vec3<f32> = normalize(in.normal);
    let lightDir : vec3<f32> = normalize(light.position - in.original_position);
    let diff : f32 = max(dot(norm, lightDir), 0.0);
    let diffuse : vec3<f32> = light.diffuse * (diff * cube_material[material_index].diffuse);
    
    // specular
    let viewDir : vec3<f32> = normalize(camera_data.position - in.original_position);
    let reflectDir : vec3<f32> = reflect(-lightDir, norm);  
    let spec : f32 = pow(max(dot(viewDir, reflectDir), 0.0), cube_material[material_index].shininess);
    let specular : vec3<f32> = light.specular * (spec * cube_material[material_index].specular);  
        
    return ambient + diffuse + specular;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    
    var result : vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    for (var i = 0; i < 2; i = i + 1) {
        result += calc_light(in, light_data[i]);
    }

    return vec4<f32>(result, 1.0);
}
