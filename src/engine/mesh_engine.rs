use crate::engine::builders::pipeline_layout_builder::PipelineLayoutBuilder;
use crate::engine::buffers::{uniform_buffer::{UniformBuffer, SetUniformBuffer}, storage_buffer::{StorageBuffer, SetStorageBuffer}};
use super::camera::third_person_camera::ThirdPersonCamera;
use super::entity_data;
use super::env::light::Bufferable;
use super::models::instance_data::{PositionInstanceData, InstanceData};
use super::models::vertex::{ModelVertex, VertexType, VertexData};
use crate::engine::builders::pipeline_builder::PipelineBuilder;
use crate::engine::builders;
use crate::engine::models::rendering::DrawModel;

pub struct MeshEngine {
    uniform_buffers: Vec<UniformBuffer>,
    storage_buffers: Vec<StorageBuffer>,
    pipelines: Vec<wgpu::RenderPipeline>,

    pub frames_draw_calls : usize,
}

impl MeshEngine {
    pub fn init(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera: &ThirdPersonCamera,
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
        let pipeline_layout = pipeline_layout_builder.build(device);

        let instanced_pipeline = PipelineBuilder::new()
            .add_vertex_buffer_layout(ModelVertex::desc())
            .add_vertex_buffer_layout(PositionInstanceData::desc())
            .set_primitive_state(Some(wgpu::Face::Back))
            .set_wireframe_mode(false)  
            .set_vertex_shader(device, "./shaders/cube.wgsl", VertexType::Vertex)
            .set_fragment_shader(device, "./shaders/cube.wgsl", &config.format)
            .set_pipeline_layout(pipeline_layout)
            .build(device);

        let mut pipeline_layout_builder = PipelineLayoutBuilder::new()
            .add_bind_group_layout(&camera_uniform.bind_group_layout)
            .add_bind_group_layout(&light_data.bind_group_layout);

        for model in &entity_data.models {
            if model.uniform_buffer.is_some() {
                pipeline_layout_builder = pipeline_layout_builder.add_bind_group_layout(&model.uniform_buffer.as_ref().unwrap().bind_group_layout);
            }
            pipeline_layout_builder = pipeline_layout_builder.add_bind_group_layout(&model.material_buffer.bind_group_layout);
        }
        let normal_pipeline_layout = pipeline_layout_builder.build(device);
        
        let normal_pipeline = PipelineBuilder::new()
            .add_vertex_buffer_layout(ModelVertex::desc())
            .set_primitive_state(Some(wgpu::Face::Back))
            .set_wireframe_mode(false)  
            .set_vertex_shader(device, "./shaders/fish.wgsl", VertexType::Vertex)
            .set_fragment_shader(device, "./shaders/fish.wgsl", &config.format)
            .set_pipeline_layout(normal_pipeline_layout)
            .build(device);

        MeshEngine {
            pipelines: vec![instanced_pipeline, normal_pipeline],
            uniform_buffers: vec![camera_uniform],
            storage_buffers: vec![light_data],
            frames_draw_calls : 0,
        }
    }

    pub fn update(&mut self, device: &wgpu::Device, camera: &ThirdPersonCamera, entity_data : &entity_data::EntityData) {
        self.uniform_buffers[0] = camera.as_uniform_buffer(device);
        self.storage_buffers[0] = entity_data.lights.as_storage_buffer(device);
    }

    pub fn render(&mut self, view: &wgpu::TextureView, depth_texture_view : &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder, entity_data : &entity_data::EntityData) {
        self.frames_draw_calls = entity_data.instanced_models.len() + entity_data.models.len();

        {
            let mut rpass = builders::pipeline_builder::create_render_pass(view, depth_texture_view, encoder);

            let mut bind_index_offset = 0;
            rpass.set_pipeline(&self.pipelines[0]);
            for i in 0..self.uniform_buffers.len() {
                rpass.set_uniform_buffer(i as u32, &self.uniform_buffers[i]);
            }
            bind_index_offset += self.uniform_buffers.len();
            for i in 0..self.storage_buffers.len() {
                rpass.set_storage_buffer((bind_index_offset + i) as u32, &self.storage_buffers[i]);
            }
            bind_index_offset += self.storage_buffers.len();
            for i in 0..entity_data.instanced_models.len() {
                rpass.draw_model_instanced((bind_index_offset + i) as u32, &entity_data.instanced_models[i]);
            }
            
            bind_index_offset = 0;
            rpass.set_pipeline(&self.pipelines[1]);
            for i in 0..self.uniform_buffers.len() {
                rpass.set_uniform_buffer(i as u32, &self.uniform_buffers[i]);
            }
            bind_index_offset += self.uniform_buffers.len();
            for i in 0..self.storage_buffers.len() {
                rpass.set_storage_buffer((bind_index_offset + i) as u32, &self.storage_buffers[i]);
            }
            bind_index_offset += self.storage_buffers.len();
            rpass.set_uniform_buffer(bind_index_offset as u32, entity_data.models[0].uniform_buffer.as_ref().unwrap()); 
            bind_index_offset += 1;
            for i in 0..entity_data.models.len() {
                rpass.draw_model((bind_index_offset + i) as u32, &entity_data.models[i]);
            }

        }
    }
}
 