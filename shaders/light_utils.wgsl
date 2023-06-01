
fn calc_light_terrain(in: TerrainVertexOutput, light : Light, material_id : i32) -> vec3<f32> {

    let ambient_strength = 0.2;
    let ambient : vec3<f32> = light.ambient * ambient_strength * materials[material_id].ambient;
  	
    // diffuse 
    let norm : vec3<f32> = normalize(in.normal);
    let lightDir : vec3<f32> = normalize(light.position - in.original_position);
    let diff : f32 = max(dot(norm, lightDir), 0.0);
    let diffuse : vec3<f32> = light.diffuse * (diff * materials[material_id].diffuse);
    
    // specular
    let viewDir : vec3<f32> = normalize(camera_data.position - in.original_position);
    let reflectDir : vec3<f32> = reflect(-lightDir, norm);  
    let spec : f32 = pow(max(dot(viewDir, reflectDir), 0.0), materials[material_id].shininess);
    let specular : vec3<f32> = light.specular * (spec * materials[material_id].specular);  
    
    return ambient + diffuse + specular;
}