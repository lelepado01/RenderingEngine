use bytemuck::{Zeroable, Pod};

use crate::engine::utils::array_extentions::ToArray4;

use super::{VertexData, Parsable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct StandardModelVertex {
    pub _pos: [f32; 4],
    pub _normal: [f32; 4],
    pub _tex_coord: [f32; 2],
}

impl StandardModelVertex {
    #[allow(dead_code)]
    pub fn new(pos : [f32; 3], normal : [f32; 3]) -> Self {
        StandardModelVertex {
            _pos: pos.to_arr4(),
            _normal: normal.to_arr4(),
            _tex_coord: [0.0, 0.0],
        }
    }
}

impl VertexData for StandardModelVertex {

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<StandardModelVertex>() as wgpu::BufferAddress,
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
                    offset: 2 * std::mem::size_of::<[f32;4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }
    }
    
}

impl Parsable for StandardModelVertex {
    fn from_mesh(index : usize, mesh : &tobj::Mesh) -> Self {
        if mesh.normals.len() == 0 {
            return StandardModelVertex {
                _pos: [ mesh.positions[index * 3], mesh.positions[index * 3 + 1], mesh.positions[index * 3 + 2], 1.0],
                _tex_coord: [mesh.texcoords[index * 2], mesh.texcoords[index * 2 + 1]],
                _normal: [0.0, 0.0, 0.0, 1.0],
            }
        } else if mesh.texcoords.len() == 0 {
            return StandardModelVertex {
                _pos: [mesh.positions[index * 3],mesh.positions[index * 3 + 1],mesh.positions[index * 3 + 2],1.0],
                _tex_coord: [0.0, 0.0],
                _normal: [mesh.normals[index * 3],mesh.normals[index * 3 + 1],mesh.normals[index * 3 + 2],1.0],
            };
        } else {
            return StandardModelVertex {
                _pos: [ mesh.positions[index * 3], mesh.positions[index * 3 + 1], mesh.positions[index * 3 + 2], 1.0],
                _tex_coord: [mesh.texcoords[index * 2], mesh.texcoords[index * 2 + 1]],
                _normal: [ mesh.normals[index * 3], mesh.normals[index * 3 + 1], mesh.normals[index * 3 + 2], 1.0],
            }; 
        }
        
    }
}