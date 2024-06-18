use cgmath::{vec3, InnerSpace, Vector3};

use crate::engine::builders::pipeline_layout_builder::PipelineLayoutBuilder;
use crate::engine::buffers::{uniform_buffer::{UniformBuffer, SetUniformBuffer}, storage_buffer::{StorageBuffer, SetStorageBuffer}};
use super::camera::fps_camera::FpsCamera;
use super::camera::Camera;
use super::models::instance::instance_data::PositionInstanceData;
use super::models::instance::{VertexData, VertexType};
use super::models::instance::voxel_vertex::VoxelVertex;
use crate::engine::builders::pipeline_builder::PipelineBuilder;
use crate::engine::builders;
use crate::engine::models::rendering::DrawModel;
use super::models::voxel_face_model::{VoxelFaceModel, VoxelFace};

const BACKGROUND_COLOR: [f32; 4] = [ 0.0, 0.0, 0.0, 1.0 ];

const DIRECTION_VECTORS : [Vector3<f32>; 6] = [
    vec3(0.0, 1.0, 0.0), // TOP
    vec3(0.0, -1.0, 0.0), // BOTTOM
    vec3(1.0, 0.0, 0.0), // RIGHT
    vec3(-1.0, 0.0, 0.0), // LEFT
    vec3(0.0, 0.0, 1.0), // BACK
    vec3(0.0, 0.0, -1.0), // FRONT
];

pub struct VoxelEngine {
    uniform_buffers: Vec<UniformBuffer>,
    storage_buffers: Vec<StorageBuffer>,
    pipelines: Vec<wgpu::RenderPipeline>,
    voxel_models: Vec<VoxelFaceModel>,
}

impl VoxelEngine {
    pub fn init<T>(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera: &T,
    ) -> Self 
        where T : Camera
    {
        let camera_uniform = camera.as_uniform_buffer(device);

        let pipeline_layout_builder = PipelineLayoutBuilder::new()
            .add_bind_group_layout(&camera_uniform.bind_group_layout); 

        let mut voxels : Vec<PositionInstanceData> = Vec::new();
        for i in 0..100 {
            for j in 0..100 {
                for z in 0..100 {
                    let x = i as f32;
                    let y = j as f32;
                    let z = z as f32;

                    voxels.push(PositionInstanceData{position:[x, y, z, 1.0]});
                }
            }
        }

        let model1 = VoxelFaceModel::new(device, VoxelFace::Bottom, voxels.clone());
        let model2 = VoxelFaceModel::new(device, VoxelFace::Top, voxels.clone());
        let model3 = VoxelFaceModel::new(device, VoxelFace::Left, voxels.clone());
        let model4 = VoxelFaceModel::new(device, VoxelFace::Right, voxels.clone());
        let model5 = VoxelFaceModel::new(device, VoxelFace::Front, voxels.clone());
        let model6 = VoxelFaceModel::new(device, VoxelFace::Back, voxels.clone());

        let pipeline_layout = pipeline_layout_builder.build(device);

        let instanced_pipeline = PipelineBuilder::new()
            .add_vertex_buffer_layout(VoxelVertex::desc())
            .add_vertex_buffer_layout(PositionInstanceData::desc())
            .set_primitive_state(Some(wgpu::Face::Back))
            .set_wireframe_mode(false)  
            .set_vertex_shader(device, "./shaders/cube.wgsl", VertexType::InstancedVertex)
            .set_fragment_shader(device, "./shaders/cube.wgsl", &config.format)
            .set_pipeline_layout(pipeline_layout)
            .build(device);

        VoxelEngine {
            pipelines: vec![instanced_pipeline],
            uniform_buffers: vec![camera_uniform],
            storage_buffers: vec![],
            voxel_models: vec![model1, model2, model3, model4, model5, model6],
        }
    }

    pub fn update<T>(
        &mut self, 
        device: &wgpu::Device, 
        camera: &T, 
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
        camera: &FpsCamera,
    ) {
        let mut rpass = builders::pipeline_builder::create_render_pass(view, depth_texture, encoder, BACKGROUND_COLOR);

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

        let camera_dir = camera.forward; 
        for (i, direction) in DIRECTION_VECTORS.iter().enumerate() {
            if direction.dot(camera_dir) >= 0.0 {
                rpass.draw_voxel_instanced(bind_index_offset as u32, &self.voxel_models[i]);
            }
        }
    }
}
 