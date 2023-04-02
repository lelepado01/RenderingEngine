// use crate::engine::builders::pipeline_bind_group_layout_builder::{BindGroupLayoutBuilder, LayoutEntryType, EntryVisibility};
use crate::engine::utils::array_extentions::ToArray4;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightData {
    pub position : [f32; 3],
    pub ambient : [f32; 3],
    pub diffuse : [f32; 3],
    pub specular :[f32; 3],
}

impl LightData {
    pub fn new() -> Self {

        Self {
            position : [15.0, 0.0, 15.0],
            ambient : [0.5, 0.5, 0.5],
            diffuse : [0.2, 0.1, 0.1],
            specular : [0.1, 0.1, 0.1],
        }
    }

    pub fn size(&self) -> u64 {
        std::mem::size_of::<f32>() as u64 * 4 * 4
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