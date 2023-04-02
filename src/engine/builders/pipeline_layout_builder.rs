
pub struct PipelineLayoutBuilder<'a> {
    bind_group_layouts : Vec<&'a wgpu::BindGroupLayout>,
}

impl<'a> PipelineLayoutBuilder<'a> {
    
    pub fn new() -> Self {
        Self {
            bind_group_layouts : Vec::new(),
        }
    }

    pub fn add_bind_group_layout(mut self, bind_group_layout : &'a wgpu::BindGroupLayout) -> Self {
        self.bind_group_layouts.push(bind_group_layout);
        self
    }

    pub fn build(self, device : &wgpu::Device) -> wgpu::PipelineLayout {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: self.bind_group_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        pipeline_layout
    }

}
