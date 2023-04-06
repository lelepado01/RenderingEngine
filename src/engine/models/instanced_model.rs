use crate::engine::buffers::{self, storage_buffer};
use super::{mesh::Mesh, instance_data::InstanceData};

pub struct InstancedModel {
    pub meshes: Vec<Mesh>,
    pub material_buffer: storage_buffer::StorageBuffer,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

impl InstancedModel {
    pub fn new<T>(
        device: &wgpu::Device,
        meshes: Vec<Mesh>,
        material_buffer: storage_buffer::StorageBuffer,
        instances: Vec<T>,
    ) -> Self 
        where T : InstanceData + bytemuck::Pod + bytemuck::Zeroable 
    {
        let instance_buffer = buffers::create_buffer(
            device,
            buffers::BufferType::Instance, 
            &instances,
        );

        Self {
            meshes,
            material_buffer,
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

    pub fn load_model<T>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        instances: Vec<T>,
    ) -> anyhow::Result<Self> 
        where T : InstanceData + bytemuck::Pod + bytemuck::Zeroable
    {
        let model = super::loading::load_model(device, queue, path).expect("Failed to load model");
        
        Ok(Self::new(
            device, 
            model.meshes, 
            model.material_buffer, 
            instances
        ))
    }
}