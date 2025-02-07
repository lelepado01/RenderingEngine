use bytemuck::{Zeroable, Pod};

use super::VertexData;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct InstanceData {
    pub position: [f32; 4], 
    pub size : [f32; 4], 
}

impl VertexData for InstanceData {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
}