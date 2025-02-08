use super::{storage_buffer::StorageBuffer, uniform_buffer::UniformBuffer};


pub trait AsUniformBuffer {
    fn as_uniform_buffer(&self, device : &wgpu::Device) -> UniformBuffer; 
}

pub trait AsStorageBuffer {
    fn as_storage_buffer(&self, device : &wgpu::Device) -> StorageBuffer; 
}