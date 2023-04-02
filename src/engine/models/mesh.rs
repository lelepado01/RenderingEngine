
pub struct Mesh {
    pub vertex_data : wgpu::Buffer,
    pub index_data : wgpu::Buffer,
    pub material_index : usize,
    pub num_elements: u32,
}