use super::material::{UnTexturedMaterial, TexturedMaterial};


pub fn parse_untextured_material(
    mat : tobj::Material, 
) -> anyhow::Result<UnTexturedMaterial> {

    Ok(UnTexturedMaterial {
        ambient: [mat.ambient[0], mat.ambient[1], mat.ambient[2], 1.0],  
        diffuse : [mat.diffuse[0], mat.diffuse[1], mat.diffuse[2], 1.0],
        specular : [mat.specular[0], mat.specular[1], mat.specular[2], 1.0],
        shininess: mat.shininess,
    })
}

pub fn parse_textured_material(
    mat : tobj::Material, 
    device : &wgpu::Device,
    queue : &wgpu::Queue,
) -> anyhow::Result<TexturedMaterial> {

    println!("Loading texture: {}...", mat.diffuse_texture);
    println!("Device: {:?}", device);
    println!("Queue: {:?}", queue);

    todo!("Implement texture loading")

    // let mut texture_view = None; 
    // if mat.diffuse_texture != "" {
    //     texture_view = Some(TextureBuilder::new(&mat.diffuse_texture, TextureType::Texture2D)
    //         .set_dimensions(2)
    //         .set_format(wgpu::TextureFormat::Rgba8UnormSrgb)
    //         .set_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
    //         .build(device, queue));
    // }

    // let mut normal_texture_view = None; 
    // if mat.normal_texture != "" { 
    //     normal_texture_view = Some(TextureBuilder::new(&mat.normal_texture, TextureType::Texture2D)
    //     .set_dimensions(2)
    //     .set_format(wgpu::TextureFormat::Rgba8UnormSrgb)
    //     .set_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
    //     .build(device, queue));
    // }
    // let sampler = create_sampler(&device);

    // let mut layout_builder = BindGroupLayoutBuilder::new(); 
    // let mut bind_group_builder = builders::pipeline_bind_group_builder::BindGroupBuilder::new(); 
    // if let Some(diffuse_texture) = texture_view.as_ref() {
    //     layout_builder
    //         .add_entry(LayoutEntryType::Texture, EntryVisibility::Fragment, 0)
    //         .add_entry(LayoutEntryType::Sampler, EntryVisibility::Fragment, 0);

    //     bind_group_builder.add_texture_entry(diffuse_texture);
    //     bind_group_builder.add_sampler_entry(&sampler);
    // }
    // if let Some(normal_texture) = normal_texture_view.as_ref() {
    //     layout_builder
    //         .add_entry(LayoutEntryType::Texture, EntryVisibility::Fragment, 0)
    //         .add_entry(LayoutEntryType::Sampler, EntryVisibility::Fragment, 0);

    //     bind_group_builder.add_texture_entry(normal_texture);
    //     bind_group_builder.add_sampler_entry(&sampler);
    // }
    // let data = vec![
    //     ambient,
    //     diffuse,
    //     specular,
    //     [mat.shininess, mat.dissolve, mat.optical_density, 0.0],
    // ];
    // let size = std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress * data.len() as wgpu::BufferAddress;
    // let mat_buffer = buffers::create_buffer(device, buffers::BufferType::Storage, &data); 
    // if ambient != [0.0, 0.0, 0.0, 1.0] {
    //     layout_builder.add_entry(LayoutEntryType::StorageBuffer, EntryVisibility::Fragment, size);
    //     bind_group_builder.add_storage_buffer_entry(&mat_buffer, size);
    // }

    // let layout = layout_builder.build(device);
    // let bind_group = bind_group_builder.build(device, &layout);

    // Ok(TexturedMaterial {
        // diffuse_texture: texture_view,
        // normal_texture: normal_texture_view,
        // bind_group,
        // bind_group_layout: layout,
    // })
}
