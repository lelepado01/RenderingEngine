use crate::engine::buffers::material_buffer::MaterialBuffer;
use crate::engine::buffers::{self, storage_buffer::StorageBuffer};
use crate::engine::utils::array_math::Add;
use crate::engine::utils::array_math::ScalarDiv;

use super::{model::Model, material::{UnTexturedMaterial, TexturedMaterial}, mesh::Mesh, vertices::{instanced_vertex::InstancedModelVertex, standard_vertex::StandardModelVertex}};

pub fn load_model_instanced(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    model_path: &str,
) -> anyhow::Result<Model> {

    let (models, materials) = tobj::load_obj(
        model_path, 
        &tobj::LoadOptions { triangulate: true, single_index: true, ..Default::default()}
    ).expect("Failed to OBJ load file");

    let mut obj_untextured_materials : Vec<UnTexturedMaterial> = Vec::new();
    let mut obj_textured_materials : Vec<TexturedMaterial> = Vec::new();

    for m in materials.expect("Failed to load materials") {
        if m.diffuse_texture != "" {
            let material = parse_textured_material(m, device, queue).expect("Failed to parse material");
            obj_textured_materials.push(material); 
        } else {
            let material = parse_untextured_material(m).expect("Failed to parse material");
            obj_untextured_materials.push(material); 
        }
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| parse_instanced_vertex(i, &m.mesh))
                .collect::<Vec<_>>();

            let vertex_buffer = buffers::create_buffer(device, buffers::BufferType::Vertex, &vertices);
            let index_buffer = buffers::create_buffer(device, buffers::BufferType::Index, &m.mesh.indices);

            if vertices[0]._normal == [0.0, 0.0, 0.0, 1.0] {
                for i in (0..m.mesh.indices.len()-3).step_by(3) {
                    let v1 = vertices[m.mesh.indices[i] as usize]._pos;
                    let v2 = vertices[m.mesh.indices[i + 1] as usize]._pos;
                    let v3 = vertices[m.mesh.indices[i + 2] as usize]._pos;

                    let normal = calculate_face_normal(v1, v2, v3);

                    vertices[m.mesh.indices[i] as usize]._normal.add(normal);
                    vertices[m.mesh.indices[i + 1] as usize]._normal.add(normal);
                    vertices[m.mesh.indices[i + 2] as usize]._normal.add(normal);
                }
            }

            for i in 0..vertices.len() {
                vertices[i]._normal = vertices[i]._normal.scalar_div(3.0);
            }

            Mesh {
                vertex_data: vertex_buffer,
                index_data: index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material_index: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

        let mut data = Vec::new();
        for material in obj_untextured_materials {
            data.push(material.ambient);
            data.push(material.diffuse);
            data.push(material.specular);
            data.push([material.shininess, 0.0, 0.0, 0.0]);
        }
        let size = (std::mem::size_of::<[f32; 4]>() * data.len()) as wgpu::BufferAddress;
        let buffer = StorageBuffer::new(device, &data, size);

    Ok(Model { 
        meshes, 
        material_buffer: None,
        instance_materials_buffer: Some(buffer), 
        uniform_buffer: None
    })
}

pub fn load_model_standard(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    model_path: &str,
) -> anyhow::Result<Model> {

    let (models, materials) = tobj::load_obj(
        model_path, 
        &tobj::LoadOptions { triangulate: true, single_index: true, ..Default::default()}
    ).expect("Failed to OBJ load file");

    let mut obj_untextured_materials : Vec<UnTexturedMaterial> = Vec::new();
    let mut obj_textured_materials : Vec<TexturedMaterial> = Vec::new();

    for m in materials.expect("Failed to load materials") {
        if m.diffuse_texture != "" {
            let material = parse_textured_material(m, device, queue).expect("Failed to parse material");
            obj_textured_materials.push(material); 
        } else {
            let material = parse_untextured_material(m).expect("Failed to parse material");
            obj_untextured_materials.push(material); 
        }
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| parse_standard_vertex(i, &m.mesh))
                .collect::<Vec<_>>();

            let vertex_buffer = buffers::create_buffer(device, buffers::BufferType::Vertex, &vertices);
            let index_buffer = buffers::create_buffer(device, buffers::BufferType::Index, &m.mesh.indices);

            if vertices[0]._normal == [0.0, 0.0, 0.0, 1.0] {
                for i in (0..m.mesh.indices.len()-3).step_by(3) {
                    let v1 = vertices[m.mesh.indices[i] as usize]._pos;
                    let v2 = vertices[m.mesh.indices[i + 1] as usize]._pos;
                    let v3 = vertices[m.mesh.indices[i + 2] as usize]._pos;

                    let normal = calculate_face_normal(v1, v2, v3);

                    vertices[m.mesh.indices[i] as usize]._normal.add(normal);
                    vertices[m.mesh.indices[i + 1] as usize]._normal.add(normal);
                    vertices[m.mesh.indices[i + 2] as usize]._normal.add(normal);
                }
            }

            for i in 0..vertices.len() {
                vertices[i]._normal = vertices[i]._normal.scalar_div(3.0);
            }

            Mesh {
                vertex_data: vertex_buffer,
                index_data: index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material_index: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

        // let mut data = Vec::new();
        // for material in obj_untextured_materials {
        //     data.push(material.ambient);
        //     data.push(material.diffuse);
        //     data.push(material.specular);
        //     data.push([material.shininess, 0.0, 0.0, 0.0]);
        // }
        // let size = (std::mem::size_of::<[f32; 4]>() * data.len()) as wgpu::BufferAddress;
        // let buffer = StorageBuffer::new(device, &data, size);

        let material_buffer = MaterialBuffer::new(device, &obj_untextured_materials);


    Ok(Model { 
        meshes, 
        material_buffer: Some(material_buffer),
        instance_materials_buffer: None,
        uniform_buffer: None
    })
}


pub fn calculate_face_normal(v1 : [f32; 4], v2 : [f32; 4], v3 : [f32; 4]) -> [f32; 4] {
    let mut normal = [0.0, 0.0, 0.0, 1.0];

    let mut v1v2 = [0.0, 0.0, 0.0];
    let mut v1v3 = [0.0, 0.0, 0.0];

    v1v2[0] = v2[0] - v1[0];
    v1v2[1] = v2[1] - v1[1];
    v1v2[2] = v2[2] - v1[2];

    v1v3[0] = v3[0] - v1[0];
    v1v3[1] = v3[1] - v1[1];
    v1v3[2] = v3[2] - v1[2];

    normal[0] = v1v2[1] * v1v3[2] - v1v2[2] * v1v3[1];
    normal[1] = v1v2[2] * v1v3[0] - v1v2[0] * v1v3[2];
    normal[2] = v1v2[0] * v1v3[1] - v1v2[1] * v1v3[0];

    let mut length = 0.0;
    length += normal[0] * normal[0];
    length += normal[1] * normal[1];
    length += normal[2] * normal[2];
    length = length.sqrt();

    normal[0] = normal[0] / length;
    normal[1] = normal[1] / length;
    normal[2] = normal[2] / length;

    return normal;
}

pub fn parse_instanced_vertex(index : usize, mesh : &tobj::Mesh) -> InstancedModelVertex {

    if mesh.normals.len() == 0 {
        return InstancedModelVertex {
            _pos: [
                mesh.positions[index * 3],
                mesh.positions[index * 3 + 1],
                mesh.positions[index * 3 + 2],
                1.0,
            ],
            _tex_coord: [mesh.texcoords[index * 2], mesh.texcoords[index * 2 + 1]],
            _normal: [0.0, 0.0, 0.0, 1.0],
        }
    } else if mesh.texcoords.len() == 0 {
        return InstancedModelVertex {
            _pos: [
                mesh.positions[index * 3],
                mesh.positions[index * 3 + 1],
                mesh.positions[index * 3 + 2],
                1.0,
            ],
            _tex_coord: [0.0, 0.0],
            _normal: [
                mesh.normals[index * 3],
                mesh.normals[index * 3 + 1],
                mesh.normals[index * 3 + 2],
                1.0,
            ],
        };
    } else {
        return InstancedModelVertex {
            _pos: [
                mesh.positions[index * 3],
                mesh.positions[index * 3 + 1],
                mesh.positions[index * 3 + 2],
                1.0,
            ],
            _tex_coord: [mesh.texcoords[index * 2], mesh.texcoords[index * 2 + 1]],
            _normal: [
                mesh.normals[index * 3],
                mesh.normals[index * 3 + 1],
                mesh.normals[index * 3 + 2],
                1.0,
            ],
        }; 
    }
}

pub fn parse_standard_vertex(index : usize, mesh : &tobj::Mesh) -> StandardModelVertex {

    if mesh.normals.len() == 0 {
        return StandardModelVertex {
            _pos: [
                mesh.positions[index * 3],
                mesh.positions[index * 3 + 1],
                mesh.positions[index * 3 + 2],
                1.0,
            ],
            _tex_coord: [mesh.texcoords[index * 2], mesh.texcoords[index * 2 + 1]],
            _normal: [0.0, 0.0, 0.0, 1.0],
        }
    } else if mesh.texcoords.len() == 0 {
        return StandardModelVertex {
            _pos: [
                mesh.positions[index * 3],
                mesh.positions[index * 3 + 1],
                mesh.positions[index * 3 + 2],
                1.0,
            ],
            _tex_coord: [0.0, 0.0],
            _normal: [
                mesh.normals[index * 3],
                mesh.normals[index * 3 + 1],
                mesh.normals[index * 3 + 2],
                1.0,
            ],
        };
    } else {
        return StandardModelVertex {
            _pos: [
                mesh.positions[index * 3],
                mesh.positions[index * 3 + 1],
                mesh.positions[index * 3 + 2],
                1.0,
            ],
            _tex_coord: [mesh.texcoords[index * 2], mesh.texcoords[index * 2 + 1]],
            _normal: [
                mesh.normals[index * 3],
                mesh.normals[index * 3 + 1],
                mesh.normals[index * 3 + 2],
                1.0,
            ],
        }; 
    }
}

pub fn parse_untextured_material(
    mat : tobj::Material, 
) -> anyhow::Result<UnTexturedMaterial> {

    Ok(UnTexturedMaterial {
        ambient: [mat.ambient[0], mat.ambient[1], mat.ambient[2], 1.0],  
        diffuse : [mat.diffuse[0], mat.diffuse[1], mat.diffuse[2], 1.0],
        specular : [mat.specular[0], mat.specular[1], mat.specular[2], 1.0],
        shininess: mat.shininess,
    })
}

pub fn parse_textured_material(
    mat : tobj::Material, 
    device : &wgpu::Device,
    queue : &wgpu::Queue,
) -> anyhow::Result<TexturedMaterial> {

    println!("Loading texture: {}...", mat.diffuse_texture);
    println!("Device: {:?}", device);
    println!("Queue: {:?}", queue);

    todo!("Implement texture loading")

    // let mut texture_view = None; 
    // if mat.diffuse_texture != "" {
    //     texture_view = Some(TextureBuilder::new(&mat.diffuse_texture, TextureType::Texture2D)
    //         .set_dimensions(2)
    //         .set_format(wgpu::TextureFormat::Rgba8UnormSrgb)
    //         .set_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
    //         .build(device, queue));
    // }

    // let mut normal_texture_view = None; 
    // if mat.normal_texture != "" { 
    //     normal_texture_view = Some(TextureBuilder::new(&mat.normal_texture, TextureType::Texture2D)
    //     .set_dimensions(2)
    //     .set_format(wgpu::TextureFormat::Rgba8UnormSrgb)
    //     .set_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
    //     .build(device, queue));
    // }
    // let sampler = create_sampler(&device);

    // let mut layout_builder = BindGroupLayoutBuilder::new(); 
    // let mut bind_group_builder = builders::pipeline_bind_group_builder::BindGroupBuilder::new(); 
    // if let Some(diffuse_texture) = texture_view.as_ref() {
    //     layout_builder
    //         .add_entry(LayoutEntryType::Texture, EntryVisibility::Fragment, 0)
    //         .add_entry(LayoutEntryType::Sampler, EntryVisibility::Fragment, 0);

    //     bind_group_builder.add_texture_entry(diffuse_texture);
    //     bind_group_builder.add_sampler_entry(&sampler);
    // }
    // if let Some(normal_texture) = normal_texture_view.as_ref() {
    //     layout_builder
    //         .add_entry(LayoutEntryType::Texture, EntryVisibility::Fragment, 0)
    //         .add_entry(LayoutEntryType::Sampler, EntryVisibility::Fragment, 0);

    //     bind_group_builder.add_texture_entry(normal_texture);
    //     bind_group_builder.add_sampler_entry(&sampler);
    // }
    // let data = vec![
    //     ambient,
    //     diffuse,
    //     specular,
    //     [mat.shininess, mat.dissolve, mat.optical_density, 0.0],
    // ];
    // let size = std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress * data.len() as wgpu::BufferAddress;
    // let mat_buffer = buffers::create_buffer(device, buffers::BufferType::Storage, &data); 
    // if ambient != [0.0, 0.0, 0.0, 1.0] {
    //     layout_builder.add_entry(LayoutEntryType::StorageBuffer, EntryVisibility::Fragment, size);
    //     bind_group_builder.add_storage_buffer_entry(&mat_buffer, size);
    // }

    // let layout = layout_builder.build(device);
    // let bind_group = bind_group_builder.build(device, &layout);

    // Ok(TexturedMaterial {
        // diffuse_texture: texture_view,
        // normal_texture: normal_texture_view,
        // bind_group,
        // bind_group_layout: layout,
    // })
}
