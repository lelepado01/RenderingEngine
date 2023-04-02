struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) original_position: vec3<f32>,
};

struct CameraData {
    transform: mat4x4<f32>,
    position: vec3<f32>,
};

struct Light{
    position : vec3<f32>,

    ambient : vec3<f32>,
    diffuse : vec3<f32>,
    specular : vec3<f32>,
}

struct TemplateMaterial {
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    specular: vec3<f32>,
    shininess: f32,
    dissolve: f32,
    optical_density: f32,
}

@group(0) @binding(0)
var<uniform> camera_data: CameraData;

@group(1) @binding(0)
var<storage, read_write> light_data: Light;
@group(1) @binding(1)
var<storage, read_write> light_data2: Light;

@group(2) @binding(0)
var<storage, read_write> cube_material : TemplateMaterial;

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
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) tex_coord: vec2<f32>,
    @location(3) instance_pos: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.normal = normal.xyz;
    out.tex_coords = tex_coord;
    out.position = camera_data.transform * (position + instance_pos);
    out.original_position = position.xyz + instance_pos.xyz;
    return out;
}

fn calc_light(in: VertexOutput, light : Light) -> vec3<f32> {
    let ambient_strength = 0.1;
    let ambient : vec3<f32> = light.ambient * ambient_strength * cube_material.ambient;
  	
    // diffuse 
    let norm : vec3<f32> = normalize(in.normal);
    let lightDir : vec3<f32> = normalize(light.position - in.original_position);
    let diff : f32 = max(dot(norm, lightDir), 0.0);
    let diffuse : vec3<f32> = light.diffuse * (diff * cube_material.diffuse);
    
    // specular
    let viewDir : vec3<f32> = normalize(camera_data.position - in.original_position);
    let reflectDir : vec3<f32> = reflect(-lightDir, norm);  
    let spec : f32 = pow(max(dot(viewDir, reflectDir), 0.0), cube_material.shininess);
    let specular : vec3<f32> = light.specular * (spec * cube_material.specular);  
        
    return ambient + diffuse + specular;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    
    var result : vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    result += calc_light(in, light_data);
    result += calc_light(in, light_data2);

    return vec4<f32>(result, 1.0);
}