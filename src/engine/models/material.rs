
#[derive(Debug)]
pub struct TexturedMaterial {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,

    pub diffuse_texture: std::option::Option<wgpu::TextureView>,
    pub normal_texture: std::option::Option<wgpu::TextureView>,

    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub shininess: f32,
    pub dissolve: f32,
    pub optical_density: f32,
}

pub struct UnTexturedMaterial {
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub shininess: f32,
}