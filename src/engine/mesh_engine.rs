use crate::engine::builders::pipeline_layout_builder::PipelineLayoutBuilder;
use crate::engine::buffers::{uniform_buffer::{UniformBuffer, SetUniformBuffer}, storage_buffer::{StorageBuffer, SetStorageBuffer}};
use super::entity_data;
use super::env::camera::Camera;
use super::env::light::Bufferable;
use super::models::instance_data::{PositionInstanceData, InstanceData};
use super::models::vertex::{ModelVertex, VertexType, VertexData};
use crate::engine::builders::pipeline_builder::PipelineBuilder;
use crate::engine::builders;
use crate::engine::models::rendering::DrawModel;

pub struct MeshEngine {
    uniform_buffers: Vec<UniformBuffer>,
    storage_buffers: Vec<StorageBuffer>,
    pipeline: wgpu::RenderPipeline,
}

impl MeshEngine {
    pub fn init(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera: &Camera,
        entity_data : &entity_data::EntityData,
    ) -> Self {

        let camera_uniform = camera.as_uniform_buffer(device);
        let light_data = entity_data.lights.as_storage_buffer(device);

        let mut pipeline_layout_builder = PipelineLayoutBuilder::new()
            .add_bind_group_layout(&camera_uniform.bind_group_layout)
            .add_bind_group_layout(&light_data.bind_group_layout);

        for model in &entity_data.instanced_models {
            pipeline_layout_builder = pipeline_layout_builder.add_bind_group_layout(&model.material_buffer.bind_group_layout);
        }
        // TODO: add bindings for normal models (not instanced)
        let pipeline_layout = pipeline_layout_builder.build(device);

        let pipeline = PipelineBuilder::new()
            .add_vertex_buffer_layout(ModelVertex::desc())
            .add_vertex_buffer_layout(PositionInstanceData::desc())
            .set_primitive_state(Some(wgpu::Face::Back))
            .set_wireframe_mode(false)  
            .set_vertex_shader(device, "./shaders/cube.wgsl", VertexType::Vertex)
            .set_fragment_shader(device, "./shaders/cube.wgsl", &config.format)
            .set_pipeline_layout(pipeline_layout)
            .build(device);

        MeshEngine {
            pipeline,
            uniform_buffers: vec![camera_uniform],
            storage_buffers: vec![light_data],
        }
    }

    pub fn update(&mut self, device: &wgpu::Device, camera: &Camera, entity_data : &entity_data::EntityData) {
        self.uniform_buffers[0] = camera.as_uniform_buffer(device);
        self.storage_buffers[0] = entity_data.lights.as_storage_buffer(device);
    }

    pub fn render(&mut self, view: &wgpu::TextureView, depth_texture_view : &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder, entity_data : &entity_data::EntityData,) {

        {
            let mut rpass = builders::pipeline_builder::create_render_pass(view, depth_texture_view, encoder);

            rpass.set_pipeline(&self.pipeline);
            for i in 0..self.uniform_buffers.len() {
                rpass.set_uniform_buffer(i as u32, &self.uniform_buffers[i]);
            }
            for i in 0..self.storage_buffers.len() {
                rpass.set_storage_buffer((self.uniform_buffers.len() + i) as u32, &self.storage_buffers[i]);
            }
            for i in 0..entity_data.instanced_models.len() {
                rpass.draw_model_instanced((self.uniform_buffers.len() + self.storage_buffers.len() + i) as u32, &entity_data.instanced_models[i]);
            }
        }
    }
}
