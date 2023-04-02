#![allow(dead_code)]
use std::borrow::Cow;

use bytemuck::Pod;
use wgpu::{Device, util::DeviceExt};

pub struct ComputeEngine<T> {
    compute_pipeline : wgpu::ComputePipeline,
    bind_group : wgpu::BindGroup,
    storage_buffer : wgpu::Buffer,
    staging_buffer : wgpu::Buffer,
    data : Vec<T>,
    data_size : wgpu::BufferAddress,
}

impl<T : Pod> ComputeEngine<T> {
    
    pub fn init(device : &Device, data : Vec<T>) -> Self {
        let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../../shaders/shader.wgsl"))),
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &cs_module,
            entry_point: "main",
        });    

        let slice_size = data.len() * std::mem::size_of::<u32>();
        let size = slice_size as wgpu::BufferAddress;

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Storage Buffer"),
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        });

        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
        });

        Self {
            compute_pipeline,
            bind_group,
            storage_buffer,
            staging_buffer,
            data,
            data_size : size,
        }
    }

    pub fn run(&self, device : &Device, queue : &wgpu::Queue) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.insert_debug_marker("compute collatz iterations");
            cpass.dispatch_workgroups(self.data.len() as u32, 1, 1); 
        }

        encoder.copy_buffer_to_buffer(&self.storage_buffer, 0, &self.staging_buffer, 0, self.data_size);
    
        queue.submit(Some(encoder.finish()));
    
        let buffer_slice = self.staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    
        device.poll(wgpu::Maintain::Wait);
    
        if let Some(Ok(())) = pollster::block_on(receiver.receive()) {

            let data = buffer_slice.get_mapped_range();
            let result : Vec<i32> = bytemuck::cast_slice(&data).to_vec();
    
            println!("Result: {:?}", result); 
        } else {
            panic!("failed to run compute on gpu!")
        }
    }  
}

