use crate::{engine::buffers::{self, storage_buffer::{self, StorageBuffer}}};
use super::{mesh::Mesh, vertices::{VertexData, instanced_vertex::InstancedModelVertex, Parsable, CalculateNormals}, material::{UnTexturedMaterial}};

pub struct IndirectModel {
    pub meshes: Vec<Mesh>,
    pub material_buffer: storage_buffer::StorageBuffer,
    pub indirect_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct IndirectDraw {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub base_index: u32,
    pub vertex_offset: i32,
    pub base_instance: i32,
}

#[allow(dead_code)]
impl IndirectModel {
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
                .map(|m| UnTexturedMaterial::from(m))
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

        IndirectModel { 
            meshes, 
            material_buffer: buffer,
            indirect_buffer: buffers::create_buffer(device, buffers::BufferType::Indirect, &vec![IndirectDraw{
                vertex_count: 8,
                instance_count: 1,
                base_index: 0,
                vertex_offset: 0,
                base_instance: 0,
            }]),
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
            &instances,
        ); 

        self.instance_buffer = instance_buffer;
        self.instance_count = instances.len() as u32;
    }

    pub fn get_byte_size<T>(&self) -> usize {
        self.instance_count as usize * std::mem::size_of::<T>()
    }

}