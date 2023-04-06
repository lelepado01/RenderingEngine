
use super::{mesh::Mesh};
use crate::engine::buffers::{storage_buffer::StorageBuffer, uniform_buffer::UniformBuffer};

pub struct Model {
    pub meshes : Vec<Mesh>,
    pub material_buffer : StorageBuffer,
    pub uniform_buffer : std::option::Option<UniformBuffer>,
}

impl Model {

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