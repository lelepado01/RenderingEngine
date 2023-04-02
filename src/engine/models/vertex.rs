use bytemuck::{Pod, Zeroable};

pub enum VertexType {
    Vertex,
}

pub trait VertexData {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}


#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct ModelVertex {
    pub _pos: [f32; 4],
    pub _normal: [f32; 4],
    pub _tex_coord: [f32; 2],
}

impl ModelVertex {
    pub fn new(pos : [f32; 3], normal : [f32; 3]) -> Self {
        ModelVertex {
            _pos: [pos[0], pos[1], pos[2], 1.0],
            _normal: [normal[0], normal[1], normal[2], 1.0],
            _tex_coord: [0.0, 0.0],
        }
    }
}

impl VertexData for ModelVertex {

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
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