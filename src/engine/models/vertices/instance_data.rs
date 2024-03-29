use bytemuck::{Zeroable, Pod};

use super::VertexData;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct PositionInstanceData {
    pub position: [f32; 4],
    pub material_index: [f32; 4],
}

impl VertexData for PositionInstanceData {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PositionInstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
}