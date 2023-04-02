use crate::engine::builders::buffers;
use super::buffers::BufferType;
use super::pipeline_bind_group_layout_builder::{BindGroupLayoutBuilder, EntryVisibility, LayoutEntryType};
use super::pipeline_bind_group_builder::BindGroupBuilder;

pub struct UniformBuffer {
    pub buffers: Vec<(wgpu::Buffer, u64)>,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl UniformBuffer {
    pub fn new<T>(device: &wgpu::Device, data: &Vec<T>, size : u64) -> Self
    where 
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let buffer = buffers::create_buffer(BufferType::Uniform, data, device);
        
        let uniform_layout = BindGroupLayoutBuilder::new()
            .add_entry(LayoutEntryType::UniformBuffer, EntryVisibility::All, size)
            .build(device);
        
        let bind_group = BindGroupBuilder::new()
            .add_uniform_buffer_entry(&buffer, size)
            .build(device, &uniform_layout);

        Self {
            buffers: vec![(buffer, size)],
            bind_group_layout: uniform_layout,
            bind_group: bind_group,
        }
    }

    pub fn add_binding<T>(&mut self, device: &wgpu::Device, data: &Vec<T>, size : u64) 
    where T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let new_buffer = buffers::create_buffer(BufferType::Uniform, data, device);
        
        let mut bind_group_builder = BindGroupBuilder::new(); 
        let mut bind_group_layout_builder = BindGroupLayoutBuilder::new();

        for (buffer, tsize) in self.buffers.iter() {
            bind_group_builder.add_uniform_buffer_entry(buffer, *tsize);
            bind_group_layout_builder.add_entry(LayoutEntryType::UniformBuffer, EntryVisibility::All, *tsize);
        }

        let bind_group_layout = bind_group_layout_builder
            .add_entry(LayoutEntryType::UniformBuffer, EntryVisibility::All, size)
            .build(device);

        let bind_group = bind_group_builder
            .add_uniform_buffer_entry(&new_buffer, size)
            .build(device, &bind_group_layout);

        self.buffers.push((new_buffer, size));
        self.bind_group_layout = bind_group_layout;
        self.bind_group = bind_group;
    }

    pub fn update<T>(&mut self, device: &wgpu::Device, binding_index : usize, data: &Vec<T>, size : u64) 
    where 
        T: bytemuck::Pod + bytemuck::Zeroable,
    {
        let new_buffer = buffers::create_buffer(BufferType::Uniform, data, device);

        let mut bind_group_builder = BindGroupBuilder::new(); 

        for (i, (buffer, tsize)) in self.buffers.iter().enumerate() {
            if binding_index == i {
                bind_group_builder.add_uniform_buffer_entry(&new_buffer, size);
            } else {
                bind_group_builder.add_uniform_buffer_entry(buffer, *tsize);
            }
        }

        let bind_group = bind_group_builder
            .build(device, &self.bind_group_layout);

        self.buffers[binding_index] = (new_buffer, size);
        self.bind_group = bind_group;
    }
}


pub trait SetUniformBuffer<'a> {
    fn set_uniform_buffer(&mut self, bind_group_index: u32, buffer : &'a UniformBuffer);
}

impl<'a> SetUniformBuffer<'a> for wgpu::RenderPass<'a> {
    fn set_uniform_buffer(&mut self, bind_group_index: u32, buffer : &'a UniformBuffer) {
        self.set_bind_group(bind_group_index, &buffer.bind_group, &[]);
    }
}