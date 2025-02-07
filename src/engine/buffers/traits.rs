use super::uniform_buffer::UniformBuffer;


pub trait AsUniformBuffer {
    fn as_uniform_buffer(&self, device : &wgpu::Device) -> UniformBuffer; 
}