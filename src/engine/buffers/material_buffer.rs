use crate::engine::models::material::UnTexturedMaterial;

use super::storage_buffer::StorageBuffer;


pub struct MaterialBuffer {
    pub buffers : Vec<StorageBuffer>,
}

impl MaterialBuffer {
    
    pub fn new(device: &wgpu::Device, materials: &Vec<UnTexturedMaterial>) -> Self {
        let mut buffers = Vec::new();
        for material in materials {
            let data = [
                material.ambient,
                material.diffuse,
                material.specular,
                [material.shininess, 0.0, 0.0, 0.0],
            ];
            let size = (std::mem::size_of::<[f32; 4]>() * data.len()) as wgpu::BufferAddress;
            let buffer = StorageBuffer::new(device, &vec![data], size);
            buffers.push(buffer);
        }

        Self {
            buffers,
        }
    }
}

pub trait SetMaterialBuffer<'a> {
    fn set_material_buffer(&mut self, bind_group_index: u32, material_id : usize, buffer : &'a MaterialBuffer);
}

impl<'a> SetMaterialBuffer<'a> for wgpu::RenderPass<'a> {
    fn set_material_buffer(&mut self, bind_group_index: u32, material_id : usize, buffer : &'a MaterialBuffer) {
        self.set_bind_group(bind_group_index, &buffer.buffers[material_id].bind_group, &[]);
    }
}