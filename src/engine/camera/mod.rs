
use super::buffers::uniform_buffer::UniformBuffer;


pub mod fps_camera; 
pub mod third_person_camera;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub trait Camera {
    fn get_view_projection_matrix(&self) -> Vec<[f32; 4]>; 
    fn as_uniform_buffer(&self, device : &wgpu::Device) -> UniformBuffer;
    fn get_position(&self) -> [f32; 3];
}