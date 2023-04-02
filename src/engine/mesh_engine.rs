use crate::engine::builders::pipeline_layout_builder::PipelineLayoutBuilder;
use crate::engine::builders::storage_buffer::SetStorageBuffer;
use super::builders::storage_buffer::StorageBuffer;
use super::builders::uniform_buffer::{UniformBuffer, SetUniformBuffer};
use super::env::camera::Camera;
use super::env::light::LightData;
use super::models::instance_data::{PositionInstanceData, InstanceData};
use super::models::vertex::{ModelVertex, VertexType, VertexData};
use super::models::{instanced_model};
use crate::engine::builders::pipeline_builder::PipelineBuilder;
use crate::engine::builders;
use crate::engine::models::rendering::DrawModel;

pub struct MeshEngine {
    uniform_buffers: Vec<UniformBuffer>,
    storage_buffers: Vec<StorageBuffer>,
    pipeline: wgpu::RenderPipeline,
    models: Vec<instanced_model::InstancedModel>,
}

impl MeshEngine {
    pub fn init(
        device: &wgpu::Device,
        queue : &wgpu::Queue,
        config: &wgpu::SurfaceConfiguration,
        camera: &Camera,
        light : &LightData,
        light2 : &LightData,
    ) -> Self {
        
        let mut poss = Vec::<[f32; 4]>::new();
        
        for i in 0..10 {
            for j in 0..10 {
                poss.push([2.0 * i as f32, 0.0 as f32, 2.0* j as f32, 1.0]);
            }
        }

        let instances : Vec<PositionInstanceData> = poss.into_iter().map(|x| PositionInstanceData { position: x }).collect();
        let model = instanced_model::InstancedModel::load_model(
            device, 
            queue,
            "assets/cube.obj", 
            instances,
        ).expect("Failed to create OBJ model"); 

        let camera_data = camera.get_camera_data();
        let buffer_size = std::mem::size_of::<[f32; 4]>() * camera_data.len();
        let camera_uniform = UniformBuffer::new(&device, &camera_data, buffer_size as u64); 
        
        let mut light_data = StorageBuffer::new(&device, &light.as_vec(), light.size() as u64); 
        light_data.add_binding(device, &light2.as_vec(), light2.size() as u64);

        let pipeline_layout = PipelineLayoutBuilder::new()
            .add_bind_group_layout(&camera_uniform.bind_group_layout)
            .add_bind_group_layout(&light_data.bind_group_layout)
            .add_bind_group_layout(&model.materials[0].bind_group_layout)
            .build(device);

        let pipeline = PipelineBuilder::new()
            .add_vertex_buffer_layout(ModelVertex::desc())
            .add_vertex_buffer_layout(PositionInstanceData::desc())
            .set_primitive_state(Some(wgpu::Face::Back))
            .set_wireframe_mode(false)  
            .set_vertex_shader(device, "src/shaders/cube.wgsl", VertexType::Vertex)
            .set_fragment_shader(device, "src/shaders/cube.wgsl", &config.format)
            .set_pipeline_layout(pipeline_layout)
            .build(device);

        MeshEngine {
            uniform_buffers: vec![camera_uniform],
            storage_buffers: vec![light_data],
            pipeline,
            models: vec![model],
        }
    }

    pub fn update(&mut self, camera: &Camera, light : &LightData, device: &wgpu::Device) {

        let camera_data = camera.get_camera_data(); 
        let buffer_size = std::mem::size_of::<[f32; 4]>() * camera_data.len(); 

        self.uniform_buffers[0].update(device, 0, &camera_data, buffer_size as u64);

        self.storage_buffers[0].update(device, 0, &light.as_vec(), light.size() as u64);
    }

    pub fn render(&mut self, view: &wgpu::TextureView, depth_texture_view : &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {

        {
            let mut rpass = builders::pipeline_builder::create_render_pass(view, depth_texture_view, encoder);

            rpass.set_pipeline(&self.pipeline);
            for i in 0..self.uniform_buffers.len() {
                rpass.set_uniform_buffer(i as u32, &self.uniform_buffers[i]);
            }
            for i in 0..self.storage_buffers.len() {
                rpass.set_storage_buffer((self.uniform_buffers.len() + i) as u32, &self.storage_buffers[i]);
            }
            rpass.draw_model_instanced((self.uniform_buffers.len() + self.storage_buffers.len()) as u32, &self.models[0]);
        }
    }
}
