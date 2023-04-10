
pub mod instanced_vertex; 
pub mod instance_data;
pub mod standard_vertex;

pub enum VertexType {
    InstancedVertex,
    StandardVertex,
}

pub trait VertexData {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

