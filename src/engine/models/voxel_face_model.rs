use crate::engine::buffers;
use super::{mesh::Mesh, instance::{VertexData, voxel_vertex::VoxelVertex}};

pub struct VoxelFaceModel {
    pub mesh: Mesh,
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

pub enum VoxelFace {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

const VERTEX_FACE_UP : [VoxelVertex; 4] = [
    VoxelVertex { _pos: [-0.5,  0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5,  0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [-0.5,  0.5,  0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5,  0.5,  0.5,  0.5] },
];

const VERTEX_FACE_DOWN : [VoxelVertex; 4] = [
    VoxelVertex { _pos: [-0.5, -0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5, -0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [-0.5, -0.5,  0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5, -0.5,  0.5,  0.5] },
];

const VERTEX_FACE_LEFT : [VoxelVertex; 4] = [
    VoxelVertex { _pos: [-0.5, -0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [-0.5,  0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [-0.5,  0.5,  0.5,  0.5] },
    VoxelVertex { _pos: [-0.5, -0.5,  0.5,  0.5] },
];

const VERTEX_FACE_RIGHT : [VoxelVertex; 4] = [
    VoxelVertex { _pos: [ 0.5, -0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5,  0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5,  0.5,  0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5, -0.5,  0.5,  0.5] },
];

const VERTEX_FACE_FRONT : [VoxelVertex; 4] = [
    VoxelVertex { _pos: [-0.5, -0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5, -0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5,  0.5, -0.5,  0.5] },
    VoxelVertex { _pos: [-0.5,  0.5, -0.5,  0.5] },
];

const VERTEX_FACE_BACK : [VoxelVertex; 4] = [
    VoxelVertex { _pos: [-0.5, -0.5,  0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5, -0.5,  0.5,  0.5] },
    VoxelVertex { _pos: [ 0.5,  0.5,  0.5,  0.5] },
    VoxelVertex { _pos: [-0.5,  0.5,  0.5,  0.5] },
];

impl VoxelFaceModel {
    pub fn new<T>(
        device: &wgpu::Device,
        voxel_face: VoxelFace,
        instances: Vec<T>,
    ) -> Self 
        where T : VertexData + bytemuck::Pod + bytemuck::Zeroable 
    {
        
        let (vertices, indices) = match voxel_face {
            VoxelFace::Bottom => (&VERTEX_FACE_DOWN, &[0, 1, 3, 0, 3, 2]),
            VoxelFace::Top => (&VERTEX_FACE_UP, &[0, 3, 1, 0, 2, 3]),
            VoxelFace::Left => (&VERTEX_FACE_LEFT, &[0, 2, 1, 0, 3, 2]),
            VoxelFace::Right => (&VERTEX_FACE_RIGHT, &[0, 1, 2, 0, 2, 3]),
            VoxelFace::Front => (&VERTEX_FACE_FRONT, &[0, 2, 1, 0, 3, 2]),
            VoxelFace::Back => (&VERTEX_FACE_BACK, &[0, 1, 2, 0, 2, 3]),
        };

        let vertex_buffer = buffers::create_buffer(device, buffers::BufferType::Vertex, vertices);
        let index_buffer = buffers::create_buffer(device, buffers::BufferType::Index, indices);
        
        let mesh = Mesh {
            vertex_data: vertex_buffer,
            index_data: index_buffer,
            num_elements: indices.len() as u32,
        };

        let instance_buffer = buffers::create_buffer(device, buffers::BufferType::Instance, &instances); 

        VoxelFaceModel { 
            mesh, 
            instance_buffer,
            instance_count: instances.len() as u32,
        }
    }
}