use bytemuck::{Pod, Zeroable};
use crate::engine::utils::array_extentions::ToArray4;
use super::VertexData;


#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct InstancedModelVertex {
    pub _pos: [f32; 4],
    pub _normal: [f32; 4],
    pub _tex_coord: [f32; 2],
}

impl InstancedModelVertex {
    #[allow(dead_code)]
    pub fn new(pos : [f32; 3], normal : [f32; 3]) -> Self {
        InstancedModelVertex {
            _pos: pos.to_arr4(),
            _normal: normal.to_arr4(),
            _tex_coord: [0.0, 0.0],
        }
    }
}

impl VertexData for InstancedModelVertex {

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstancedModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 2*std::mem::size_of::<[f32;4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
    
}