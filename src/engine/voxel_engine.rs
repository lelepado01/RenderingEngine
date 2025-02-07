use cgmath::{vec3, Vector3};
use noise::{Perlin, NoiseFn};

use crate::engine::builders::pipeline_layout_builder::PipelineLayoutBuilder;
use crate::engine::buffers::{uniform_buffer::{UniformBuffer, SetUniformBuffer}, storage_buffer::{StorageBuffer, SetStorageBuffer}};
use super::buffers::traits::AsUniformBuffer;
use super::camera::fps_camera::FpsCamera;
use super::models::instance::instance_data::InstanceData;
use super::models::instance::{VertexData, VertexType};
use super::models::instance::voxel_vertex::VoxelVertex;
use crate::engine::builders::pipeline_builder::PipelineBuilder;
use crate::engine::builders;
use crate::engine::models::rendering::DrawModel;
use super::models::voxel_face_model::{VoxelFaceModel, VoxelFace};
use crate::engine::data::QuadtreeNode; 

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

fn generate_terrain(quadtree: &mut QuadtreeNode, size: u32, max_height: f32, scale: f64) {
    let perlin = Perlin::new(42); // Create a new Perlin noise generator

    for i in 1..=size {
        for z in 1..=size {
            let x = i as f32;
            let z = z as f32;

            // Use Perlin noise to determine height
            let noise_value = perlin.get([x as f64 * scale, z as f64 * scale]); // Get noise value
            let normalized_height = ((noise_value + 1.0) / 2.0) as f32 * max_height/5.0; // Normalize to range [0, max_height]
            
            for j in 1..normalized_height as i32{
                let pos = Vector3::new(x, j as f32, z); 
                quadtree.insert_voxel(pos);    
            }
        }
    }
}

impl VoxelEngine {
    pub fn init(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        camera: &dyn AsUniformBuffer,
        light : &dyn AsUniformBuffer
    ) -> Self {
        let camera_uniform = camera.as_uniform_buffer(device);
        let light_uniform = light.as_uniform_buffer(device); 

        let pipeline_layout_builder = PipelineLayoutBuilder::new()
            .add_bind_group_layout(&camera_uniform.bind_group_layout)
            .add_bind_group_layout(&light_uniform.bind_group_layout); 

        let size = 1024; 
        let height = 200; 
        let mut quadtree : QuadtreeNode = QuadtreeNode::new(size); 
        generate_terrain(&mut quadtree, size, height as f32, 0.01);
        let voxels : Vec<InstanceData> = quadtree.get_data(); 
        
        let model1 = VoxelFaceModel::new(device, VoxelFace::Bottom, voxels.clone());
        let model2 = VoxelFaceModel::new(device, VoxelFace::Top, voxels.clone());
        let model3 = VoxelFaceModel::new(device, VoxelFace::Left, voxels.clone());
        let model4 = VoxelFaceModel::new(device, VoxelFace::Right, voxels.clone());
        let model5 = VoxelFaceModel::new(device, VoxelFace::Front, voxels.clone());
        let model6 = VoxelFaceModel::new(device, VoxelFace::Back, voxels.clone());

        let pipeline_layout = pipeline_layout_builder.build(device);

        let instanced_pipeline = PipelineBuilder::new()
            .add_vertex_buffer_layout(VoxelVertex::desc())
            .add_vertex_buffer_layout(InstanceData::desc())
            .set_primitive_state(Some(wgpu::Face::Back))
            .set_wireframe_mode(false)  
            .set_vertex_shader(device, "./shaders/cube.wgsl", VertexType::InstancedVertex)
            .set_fragment_shader(device, "./shaders/cube.wgsl", &config.format)
            .set_pipeline_layout(pipeline_layout)
            .build(device);

        VoxelEngine {
            pipelines: vec![instanced_pipeline],
            uniform_buffers: vec![camera_uniform, light_uniform],
            storage_buffers: vec![],
            voxel_models: vec![model1, model2, model3, model4, model5, model6],
        }
    }

    pub fn update(
        &mut self, 
        device: &wgpu::Device, 
        camera: &dyn AsUniformBuffer, 
        light : &dyn AsUniformBuffer
    ) {
        self.uniform_buffers[0] = camera.as_uniform_buffer(device);
        self.uniform_buffers[1] = light.as_uniform_buffer(device);
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
            // if direction.dot(camera_dir) >= -0.5 {
                rpass.draw_voxel_instanced(bind_index_offset as u32, &self.voxel_models[i]);
            // }
        }
    }
}
 