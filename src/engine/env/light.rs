use crate::engine::{utils::array_extentions::ToArray4, buffers::storage_buffer::StorageBuffer};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightData {
    pub position : [f32; 3],
    pub ambient : [f32; 3],
    pub diffuse : [f32; 3],
    pub specular :[f32; 3],
}

impl LightData {
    pub fn new(pos : [f32; 3]) -> Self {

        Self {
            position : pos,
            ambient : [0.2, 0.2, 0.3],
            diffuse : [0.1, 0.1, 0.2],
            specular : [0.1, 0.1, 0.2],
        }
    }

    pub fn size(&self) -> u64 {
        std::mem::size_of::<[f32; 4]>() as u64 * 4
    }

    pub fn as_vec(&self) -> Vec<[f32; 4]> {
        vec![
            self.position.to_arr4(), 
            self.ambient.to_arr4(), 
            self.diffuse.to_arr4(), 
            self.specular.to_arr4()
        ]
    }
}

pub trait Bufferable {
    fn as_storage_buffer(&self, device : &wgpu::Device) -> StorageBuffer;
}

impl Bufferable for Vec<LightData> {
    fn as_storage_buffer(&self, device : &wgpu::Device) -> StorageBuffer {

        let data = self.iter().map(|light| light.as_vec()).flatten().collect::<Vec<[f32; 4]>>();
        let size = self.len() as u64 * self[0].size();

        StorageBuffer::new(&device, &data, size)
    }
}