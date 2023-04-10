
use super::{mesh::Mesh, material::{UnTexturedMaterial, TexturedMaterial}, textures::{parse_textured_material, parse_untextured_material}, vertices::{standard_vertex::StandardModelVertex, Parsable, CalculateNormals}};
use crate::engine::buffers::{uniform_buffer::UniformBuffer, material_buffer::MaterialBuffer, self};

pub struct StandardModel {
    pub meshes : Vec<Mesh>,
    pub material_buffer : MaterialBuffer,
    pub uniform_buffer : std::option::Option<UniformBuffer>,
}

impl StandardModel {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        model_path: &str,
    ) -> anyhow::Result<StandardModel> {
    
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
                    .map(|i| StandardModelVertex::from_mesh(i, &m.mesh))
                    .collect::<Vec<_>>();
    
                let vertex_buffer = buffers::create_buffer(device, buffers::BufferType::Vertex, &vertices);
                let index_buffer = buffers::create_buffer(device, buffers::BufferType::Index, &m.mesh.indices);
    
                if vertices[0]._normal == [0.0, 0.0, 0.0, 1.0] {
                    vertices.calculate_normals(&m.mesh.indices);
                }
    
                Mesh {
                    vertex_data: vertex_buffer,
                    index_data: index_buffer,
                    num_elements: m.mesh.indices.len() as u32,
                    material_index: m.mesh.material_id.unwrap_or(0),
                }
            })
            .collect::<Vec<_>>();
    
        Ok(StandardModel { 
            meshes, 
            material_buffer: MaterialBuffer::new(device, &obj_untextured_materials),
            uniform_buffer: None
        })
    }

    pub fn set_uniform_buffer(&mut self, buffer : UniformBuffer) {
        self.uniform_buffer = Some(buffer);
    }

    pub fn update_uniform_buffer<T>(&mut self, device : &wgpu::Device, data : &Vec<T>, size : u64) 
    where T : bytemuck::Pod + bytemuck::Zeroable
    {
        if let Some(buffer) = self.uniform_buffer.as_mut() {
            buffer.update(device, 0, data, size);
        }
    }

}