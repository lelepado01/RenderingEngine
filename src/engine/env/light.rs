use crate::engine::builders::pipeline_bind_group_layout_builder::{BindGroupLayoutBuilder, LayoutEntryType, EntryVisibility};


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightData {
    pub position : [f32; 4],
    pub ambient : [f32; 4],
    pub diffuse : [f32; 4],
    pub specular :[f32; 4],
}

impl LightData {
    pub fn new() -> Self {

        Self {
            position : [15.0, 0.0, 15.0, 1.0],
            ambient : [0.5, 0.5, 0.5, 0.0],
            diffuse : [0.2, 0.1, 0.1,1.0],
            specular : [0.1, 0.1, 0.1, 1.0],
        }
    }


    pub fn size(&self) -> u64 {
        std::mem::size_of::<f32>() as u64 * 4 * 4
    }

    pub fn as_buffer(&self, device : &wgpu::Device) -> wgpu::Buffer {

        let mut data : Vec<u8> = bytemuck::cast_slice(&self.position).to_vec();
        data.extend_from_slice(bytemuck::cast_slice(&self.ambient));
        data.extend_from_slice(bytemuck::cast_slice(&self.diffuse));
        data.extend_from_slice(bytemuck::cast_slice(&self.specular));

        let size = data.len() as u64;
        let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        });

        buffer
    }

    pub fn as_vec(&self) -> Vec<[f32; 4]> {
        vec![self.position, self.ambient, self.diffuse, self.specular]
    }

    pub fn get_bind_group_layout(&self, device : &wgpu::Device) -> wgpu::BindGroupLayout {
        BindGroupLayoutBuilder::new()
            .add_entry(LayoutEntryType::StorageBuffer, EntryVisibility::Fragment, self.size())
            // .add_entry(LayoutEntryType::UniformBuffer, EntryVisibility::All, self.size())
            .build(device)
    }

    pub fn get_position(&self) -> [f32; 3] {
        [self.position[0], self.position[1], self.position[2]]
    }

    pub fn set_position(&mut self, position : [f32; 3]) {
        self.position[0] = position[0];
        self.position[1] = position[1];
        self.position[2] = position[2];
    }

}