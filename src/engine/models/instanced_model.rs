use crate::engine::buffers::{self, storage_buffer::{self, StorageBuffer}};
use super::{mesh::Mesh, vertices::{VertexData, instanced_vertex::InstancedModelVertex, Parsable, CalculateNormals}, material::UnTexturedMaterial};

pub struct InstancedModel {
    pub meshes: Vec<Mesh>,
    pub material_buffer: storage_buffer::StorageBuffer,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

impl InstancedModel {
    pub fn new<T>(
        device: &wgpu::Device,
        path: &str,
        instances: Vec<T>,
    ) -> Self 
        where T : VertexData + bytemuck::Pod + bytemuck::Zeroable 
    {
        let (models, materials) = tobj::load_obj(
            path, 
            &tobj::LoadOptions { triangulate: true, single_index: true, ..Default::default()}
        ).expect("Failed to OBJ load file");

        let meshes = models
            .into_iter()
            .map(|m| {
                let mut vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| InstancedModelVertex::from_mesh(i, &m.mesh))
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

            let obj_untextured_materials : Vec<UnTexturedMaterial> = 
                materials.expect("Failed to load materials")
                    .iter()
                    .map(UnTexturedMaterial::from)
                    .collect();

            let mut data = Vec::new();
            for material in obj_untextured_materials {
                data.push(material.ambient);
                data.push(material.diffuse);
                data.push(material.specular);
                data.push([material.shininess, 0.0, 0.0, 0.0]);
            }
            let size = (std::mem::size_of::<[f32; 4]>() * data.len()) as wgpu::BufferAddress;
            let buffer = StorageBuffer::new(device, &data, size);

            let instance_buffer = buffers::create_buffer(
                device,
                buffers::BufferType::Instance, 
                &instances,
            );

        InstancedModel { 
            meshes, 
            material_buffer: buffer,
            instance_buffer,
            instance_count: instances.len() as u32,
        }
    }

    #[allow(dead_code)]
    pub fn update_instances<T>(&mut self, device: &wgpu::Device, instances: &Vec<T>) 
    where T : bytemuck::Pod + bytemuck::Zeroable
    {
        let instance_buffer = buffers::create_buffer(
            device,
            buffers::BufferType::Instance, 
            instances,
        ); 

        self.instance_buffer = instance_buffer;
        self.instance_count = instances.len() as u32;
    }
}