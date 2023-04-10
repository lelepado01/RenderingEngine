use crate::engine::buffers::{self, storage_buffer};
use super::{mesh::Mesh, vertices::VertexData};

pub struct InstancedModel {
    pub meshes: Vec<Mesh>,
    pub material_buffer: storage_buffer::StorageBuffer,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

impl InstancedModel {
    pub fn new<T>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        instances: Vec<T>,
    ) -> Self 
        where T : VertexData + bytemuck::Pod + bytemuck::Zeroable 
    {
        super::loading::load_model_instanced(device, queue, path, instances).expect("Failed to load model")
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
}