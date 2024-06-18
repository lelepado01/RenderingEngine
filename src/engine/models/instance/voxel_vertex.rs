use bytemuck::{Pod, Zeroable};
use super::VertexData;


#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct VoxelVertex {
    pub _pos: [f32; 4],
}

impl VertexData for VoxelVertex {

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VoxelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }
    }
}