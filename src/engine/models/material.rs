use crate::engine::{builders::{pipeline_bind_group_layout_builder::{BindGroupLayoutBuilder, LayoutEntryType, EntryVisibility}, self}, buffers::storage_buffer::StorageBuffer};



#[derive(Debug)]
pub struct TemplateMaterial {
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

// #[derive(Debug)]
// pub struct MaterialBuffer {
//     pub bind_group: wgpu::BindGroup,
//     pub bind_group_layout: wgpu::BindGroupLayout,
// }

// impl MaterialBuffer {
//     pub fn new(
//         device: &wgpu::Device,
//         queue: &wgpu::Queue,
//         materials: &Vec<TemplateMaterial>,
//     ) -> anyhow::Result<MaterialBuffer> {
//         // let mut layout_builder = BindGroupLayoutBuilder::new();
//         // let mut bind_group_builder = builders::pipeline_bind_group_builder::BindGroupBuilder::new();

//         // layout_builder
//         //     .add_entry(LayoutEntryType::StorageBuffer, EntryVisibility::Fragment, 0);

//         // bind_group_builder.add_storage_buffer_entry(&buffer, size);

//         // let bind_group_layout = layout_builder.build(&device);
//         // let bind_group = bind_group_builder.build(&device, &bind_group_layout);

//         Ok(MaterialBuffer {
//             bind_group: buffer.bind_group,
//             bind_group_layout: buffer.bind_group_layout,
//         })
//     }
// }