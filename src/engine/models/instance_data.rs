use bytemuck::{Zeroable, Pod};


pub trait InstanceData {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>; 
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct PositionInstanceData {
    pub position: [f32; 4],
}

impl InstanceData for PositionInstanceData {
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
            ]
        }
    }
}