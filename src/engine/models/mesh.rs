
pub struct Mesh {
    pub vertex_data : wgpu::Buffer,
    pub index_data : wgpu::Buffer,
    pub num_elements: u32,
}