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

pub struct MeshEngine {
    uniform_buffers: Vec<UniformBuffer>,
    storage_buffers: Vec<StorageBuffer>,
    instanced_pipeline: wgpu::RenderPipeline,
}

impl MeshEngine {
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
            .add_bind_group_layout(&camera_uniform.bind_group_layout)
            .add_bind_group_layout(&light_data.bind_group_layout);

        for model in &entity_data.instanced_models {
            pipeline_layout_builder = pipeline_layout_builder.add_bind_group_layout(&model.material_buffer.bind_group_layout);
        }        
        let pipeline_layout = pipeline_layout_builder.build(device);

        let instanced_pipeline = PipelineBuilder::new()
            .add_vertex_buffer_layout(InstancedModelVertex::desc())
            .add_vertex_buffer_layout(MaterialInstanceData::desc())
            .set_primitive_state(Some(wgpu::Face::Back))
            .set_wireframe_mode(false)  
            .set_vertex_shader(device, "./shaders/positional_tilemap.wgsl", VertexType::InstancedVertex)
            .set_fragment_shader(device, "./shaders/positional_tilemap.wgsl", &config.format)
            .set_pipeline_layout(pipeline_layout)
            .build(device);

        MeshEngine {
            instanced_pipeline,
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
        self.storage_buffers[0] = entity_data.lights.as_storage_buffer(device);
    }

    pub fn render(
        &mut self, 
        view : &wgpu::TextureView,
        depth_texture: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder, 
        entity_data : &entity_data::EntityData, 
        stats : &mut EngineStats
    ) {
        {
            let mut rpass = builders::pipeline_builder::create_render_pass(view, depth_texture, encoder, BACKGROUND_COLOR);

            let mut bind_index_offset = 0;
            rpass.set_pipeline(&self.instanced_pipeline);
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
        }
        
        let models_calls = entity_data.models.iter().fold(0, |acc, model| acc + model.meshes.len()); 
        stats.frames_draw_calls = entity_data.instanced_models.len() + models_calls; 
        if entity_data.instanced_models.len() > 0 {
            stats.bytes_to_gpu += self.uniform_buffers.len() * self.uniform_buffers[0].buffers.len() * self.uniform_buffers[0].buffers[0].1 as usize;
            stats.bytes_to_gpu += self.storage_buffers.len() * self.storage_buffers[0].buffers.len() * self.storage_buffers[0].buffers[0].1 as usize;
            stats.bytes_to_gpu += entity_data.instanced_models.len() * entity_data.instanced_models[0].material_buffer.buffers.len() * entity_data.instanced_models[0].material_buffer.buffers[0].1 as usize;
            stats.bytes_to_gpu += entity_data.instanced_models.iter().fold(0, |acc, model| acc + model.get_byte_size::<MaterialInstanceData>()); 
        }
        if entity_data.models.len() > 0 {
            stats.bytes_to_gpu += entity_data.models.len() * entity_data.models[0].material_buffer.buffers.len() * std::mem::size_of::<[f32; 4]>() * 4 as usize;
        }
    }
}
 