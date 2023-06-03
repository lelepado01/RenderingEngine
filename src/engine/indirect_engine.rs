use crate::engine::builders::pipeline_layout_builder::PipelineLayoutBuilder;
use crate::engine::buffers::{uniform_buffer::{UniformBuffer, SetUniformBuffer}, storage_buffer::{StorageBuffer, SetStorageBuffer}};
use crate::game::tilemap::MaterialInstanceData;
use super::camera::Camera;
use super::entity_data;
use super::env::light::Bufferable;
use super::models::vertices::{VertexData, VertexType};
use super::models::vertices::instanced_vertex::InstancedModelVertex;
use super::stats::EngineStats;
use crate::engine::builders::pipeline_builder::PipelineBuilder;
use crate::engine::builders;
use crate::engine::models::rendering::DrawModel;

const BACKGROUND_COLOR: [f32; 4] = [ 0.43, 0.72, 0.72, 1.0 ];

pub struct IndirectEngine {
    uniform_buffers: Vec<UniformBuffer>,
    storage_buffers: Vec<StorageBuffer>,
    indirect_pipeline: wgpu::RenderPipeline,
}

impl IndirectEngine {
    pub fn init<T>(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera: &T,
        entity_data : &entity_data::EntityData,
    ) -> Self 
        where T : Camera
    {
        let camera_uniform = camera.as_uniform_buffer(device);
        let light_data = entity_data.lights.as_storage_buffer(device);

        let mut pipeline_layout_builder = PipelineLayoutBuilder::new()
            .add_bind_group_layout(&camera_uniform.bind_group_layout);

        // for model in &entity_data.instanced_models {
        //     pipeline_layout_builder = pipeline_layout_builder.add_bind_group_layout(&model.material_buffer.bind_group_layout);
        // }        
        let pipeline_layout = pipeline_layout_builder.build(device);

        let indirect_pipeline = PipelineBuilder::new()
            .add_vertex_buffer_layout(InstancedModelVertex::desc())
            .add_vertex_buffer_layout(MaterialInstanceData::desc())
            .set_primitive_state(Some(wgpu::Face::Back))
            .set_wireframe_mode(false)  
            .set_vertex_shader(device, "./shaders/cube.wgsl", VertexType::InstancedVertex)
            .set_fragment_shader(device, "./shaders/cube.wgsl", &config.format)
            .set_pipeline_layout(pipeline_layout)
            .build(device);

        IndirectEngine {
            indirect_pipeline,
            uniform_buffers: vec![camera_uniform],
            storage_buffers: vec![light_data],
        }
    }

    pub fn update<T>(
        &mut self, 
        device: &wgpu::Device, 
        camera: &T, 
        entity_data : &entity_data::EntityData
    ) 
        where T : Camera
    {
        self.uniform_buffers[0] = camera.as_uniform_buffer(device);
    }

    pub fn render(
        &mut self, 
        view : &wgpu::TextureView,
        depth_texture: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder, 
        entity_data : &entity_data::EntityData, 
        stats : &mut EngineStats
    ) {
        let mut rpass = builders::pipeline_builder::create_render_pass(view, depth_texture, encoder, BACKGROUND_COLOR);

        rpass.set_pipeline(&self.indirect_pipeline);
        rpass.set_uniform_buffer(0, &self.uniform_buffers[0]);
        // rpass.set_storage_buffer(1, &self.storage_buffers[0]);
        rpass.draw_model_indirect(0, &entity_data.indirect_models[0]);
    }
}
 