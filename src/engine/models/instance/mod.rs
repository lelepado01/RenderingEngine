

pub mod voxel_vertex; 
pub mod instance_data;

pub enum VertexType {
    InstancedVertex,
}

pub trait VertexData {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}