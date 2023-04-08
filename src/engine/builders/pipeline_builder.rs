use crate::engine::{models::vertex::VertexType, shaders};


pub struct PipelineBuilder<'a> {
    vertex_type : VertexType,
    vertex_code : std::option::Option<wgpu::ShaderModule>,
    fragment_configs : std::option::Option<wgpu::TextureFormat>,
    fragment_code : std::option::Option<wgpu::ShaderModule>,
    primitive_state_cull_mode : std::option::Option<wgpu::Face>,
    wireframe_mode : bool,
    pipeline_layout : std::option::Option<wgpu::PipelineLayout>,
    vertex_buffer_layouts : Vec<wgpu::VertexBufferLayout<'a>>,
}

impl<'a> PipelineBuilder<'a> {
    
    pub fn new() -> Self {
        Self {
            vertex_type : VertexType::Vertex,
            vertex_code : None,
            fragment_configs : None,
            fragment_code : None,
            primitive_state_cull_mode : None,
            wireframe_mode : false,
            pipeline_layout : None,
            vertex_buffer_layouts : Vec::new(),
        }
    }

    pub fn set_pipeline_layout(mut self, pipeline_layout : wgpu::PipelineLayout) -> Self {
        self.pipeline_layout = Some(pipeline_layout);
        self
    }

    pub fn set_vertex_shader(mut self, device : &wgpu::Device, shader_path : &str, vertex_type : VertexType) -> Self {
        self.vertex_type = vertex_type;        
        self.vertex_code = Some(device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shaders::compile_shader(shader_path).into()),
        }));

        self
    }

    pub fn set_fragment_shader(mut self, device : &wgpu::Device, shader_path : &str, configs : &wgpu::TextureFormat) -> Self {
        let shader_mod_desc = wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shaders::compile_shader(shader_path).into()),
        };
        self.fragment_code = Some(device.create_shader_module(shader_mod_desc));
        self.fragment_configs = Some(configs.clone());

        self
    }

    pub fn set_primitive_state(mut self, cull : std::option::Option<wgpu::Face>) -> Self {
        self.primitive_state_cull_mode = cull;
        self
    }

    pub fn set_wireframe_mode(mut self, mode: bool) -> Self {
        self.wireframe_mode = mode;
        self
    }

    pub fn add_vertex_buffer_layout(mut self, layouts : wgpu::VertexBufferLayout<'a>) -> Self {
        self.vertex_buffer_layouts.push(layouts);
        self
    }

    pub fn build(self, device : &wgpu::Device) -> wgpu::RenderPipeline {
        let buffers = self.vertex_buffer_layouts.as_slice();
        let vertex_state : wgpu::VertexState = wgpu::VertexState {
            module: &self.vertex_code.unwrap(),
            entry_point: "vs_main",
            buffers: &buffers,
        };

        let fragment_state : wgpu::FragmentState = wgpu::FragmentState {
            module: &self.fragment_code.unwrap(),
            entry_point: "fs_main",
            targets: &[Some(self.fragment_configs.unwrap().into())],
        };

        let topology = if self.wireframe_mode {
            wgpu::PrimitiveTopology::LineList
        } else {
            wgpu::PrimitiveTopology::TriangleList
        };
    
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: self.pipeline_layout.as_ref(),
            vertex: vertex_state,
            fragment: Some(fragment_state),
            primitive: wgpu::PrimitiveState {
                topology, 
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, 
                cull_mode: self.primitive_state_cull_mode,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },        
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, 
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }), 
            multisample: wgpu::MultisampleState {
                count: 1, 
                mask: !0, 
                alpha_to_coverage_enabled: false,
            },
            multiview: None, 
        })
    }
}

pub fn create_render_pass<'a>(
    view : &'a wgpu::TextureView,
    depth_texture_view : &'a wgpu::TextureView,
    encoder : &'a mut wgpu::CommandEncoder,
    [r, g, b, a] : [f32; 4]
) -> wgpu::RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[
            Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(
                        wgpu::Color { r: r as f64, g: g as f64, b: b as f64, a: a as f64 }
                    ),
                    store: true,
                }
            })
        ],
        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            view: &depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        }),    
    })
}