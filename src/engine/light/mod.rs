use cgmath::Vector3;
use crate::engine::buffers::uniform_buffer::UniformBuffer;

use super::buffers::traits::AsUniformBuffer; 

pub struct DirectionalLight {
    pub direction : Vector3<f32>, 
    pub color : Vector3<f32>
}

impl DirectionalLight {
    pub fn new() -> DirectionalLight {
        DirectionalLight {
            direction: Vector3::new(0.5, -0.5, 0.5), 
            color: Vector3::new(1.0, 1.0, 0.5), 
        }
    }
}

impl AsUniformBuffer for DirectionalLight {
    fn as_uniform_buffer(&self, device : &wgpu::Device) -> UniformBuffer {
        let mut light_data = Vec::<f32>::new();
        light_data.extend::<[f32; 4]>(self.direction.extend(1.0).into());
        light_data.extend::<[f32; 4]>(self.color.extend(1.0).into());
        
        let buffer_size = std::mem::size_of::<f32>() * light_data.len();
        UniformBuffer::new(device, &light_data, buffer_size as u64)
    }
}