use crate::engine::builders::buffers;
use super::{material::TemplateMaterial, mesh::Mesh, instance_data::InstanceData};

pub struct InstancedModel {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<TemplateMaterial>,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

impl InstancedModel {
    pub fn new<T>(
        device: &wgpu::Device,
        meshes: Vec<Mesh>,
        materials: Vec<TemplateMaterial>,
        instances: Vec<T>,
    ) -> Self 
        where T : InstanceData + bytemuck::Pod + bytemuck::Zeroable 
    {
        let instance_buffer = buffers::create_buffer(
            buffers::BufferType::Instance, 
            &instances,
            device
        );

        Self {
            meshes,
            materials,
            instance_buffer,
            instance_count: instances.len() as u32,
        }
    }

    pub fn update_instances<T>(&mut self, device: &wgpu::Device, instances: Vec<T>) 
    where T : bytemuck::Pod + bytemuck::Zeroable
    {
        let instance_buffer = buffers::create_buffer(
            buffers::BufferType::Instance, 
            &instances,
            device
        ); 

        self.instance_buffer = instance_buffer;
        self.instance_count = instances.len() as u32;
    }

    pub fn load_model<T>(
        path: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        instances: Vec<T>,
    ) -> anyhow::Result<Self> 
    where T : InstanceData + bytemuck::Pod + bytemuck::Zeroable
    {

        let model = super::model::load_model(path, device, queue).expect("Failed to load model");
        Ok(Self::new(device, model.meshes, model.materials, instances))
    }
}