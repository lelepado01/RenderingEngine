

pub struct BindGroupBuilder<'a> {
    entries : Vec<wgpu::BindGroupEntry<'a>>,
}

impl<'a> BindGroupBuilder<'a> {
    pub fn new() -> Self { 
        Self {
            entries : Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_texture_entry(&mut self, texture: &'a wgpu::TextureView) -> &mut Self {
        let binding = self.entries.len() as u32;

        self.entries.push(wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(texture),
        });

        self
    }

    #[allow(dead_code)]
    pub fn add_sampler_entry(&mut self, sampler: &'a wgpu::Sampler) -> &mut Self {
        let binding = self.entries.len() as u32;

        self.entries.push(wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Sampler(sampler),
        });

        self
    }

    pub fn add_storage_buffer_entry(&mut self, buffer: &'a wgpu::Buffer, size : u64) -> &mut Self {
        let binding = self.entries.len() as u32;

        self.entries.push(wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer,
                offset: 0,
                size: wgpu::BufferSize::new(size),
            }),
        });

        self
    }

    pub fn add_uniform_buffer_entry(&mut self, buffer: &'a wgpu::Buffer, size : u64) -> &mut Self {
        let binding = self.entries.len() as u32;

        self.entries.push(wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer,
                offset: 0,
                size: wgpu::BufferSize::new(size),
            }),
        });

        self
    }    

    pub fn build(&self, device: &wgpu::Device, layout : &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: layout,
            entries: &self.entries,
            label: None,
        })
    }
}