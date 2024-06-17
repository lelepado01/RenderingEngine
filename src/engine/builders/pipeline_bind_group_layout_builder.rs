pub enum LayoutEntryType {
    UniformBuffer,
    StorageBuffer,
}

pub enum EntryVisibility {
    Vertex,
    Fragment,
    All,
}

pub struct BindGroupLayoutBuilder {
    binding_count : u32,
    entries : Vec<wgpu::BindGroupLayoutEntry>,
}

impl BindGroupLayoutBuilder {
    pub fn new() -> Self {
        Self {
            binding_count : 0,
            entries : Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry_type : LayoutEntryType, visibility : EntryVisibility, size : u64) -> &mut Self {

        let visibility = match visibility {
            EntryVisibility::Vertex => wgpu::ShaderStages::VERTEX,
            EntryVisibility::Fragment => wgpu::ShaderStages::FRAGMENT,
            EntryVisibility::All => wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
        };

        let entry = match entry_type {
            LayoutEntryType::UniformBuffer => wgpu::BindGroupLayoutEntry {
                binding: self.binding_count,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size),
                },
                count: None,
            },
            LayoutEntryType::StorageBuffer => wgpu::BindGroupLayoutEntry {
                binding: self.binding_count,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(size),
                },
                count: None,
            },
        };

        self.binding_count += 1;

        self.entries.push(entry);
        self
    }

    pub fn build(&self, device : &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: self.entries.as_slice(),
        })
    }   
}