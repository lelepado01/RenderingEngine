
use cgmath::{Vector3, InnerSpace};
use crate::engine::{engine::EngineData, buffers::uniform_buffer::UniformBuffer, utils::vector_extensions::ToPoint3};

use super::{Camera, OPENGL_TO_WGPU_MATRIX};

pub const SENSITIVITY:f32 = 0.05;

pub struct FpsCamera {
    pub position : Vector3<f32>,
    pub forward : Vector3<f32>,
    aspect_ratio : f32,

    yaw : f32,
    pitch : f32,
}


impl FpsCamera {

    pub fn new(start_pos : [f32; 3], aspect_ratio : f32) -> Self {
        Self {
            position : Vector3::from(start_pos),
            forward : Vector3::new(0.0, 0.0, -1.0),
            yaw : 0.0,
            pitch : 0.0,
            aspect_ratio,
        }
    }

    pub fn update_aspect_ratio(&mut self, engine : &EngineData) {
        let window_size = engine.get_window_size();
        self.aspect_ratio = window_size.0 as f32 / window_size.1 as f32;
    }

    pub fn update_position(&mut self, player_pos : Vector3<f32>){
        
        self.position.x = player_pos.x;
        self.position.z = player_pos.z;
        self.position.y = player_pos.y;

        self.forward = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        ).normalize();
    }

    pub fn update_rotation(&mut self, x : f32, y : f32) {
        self.yaw += x * SENSITIVITY;
        self.pitch -= y * SENSITIVITY;
    }

}

impl Camera for FpsCamera {
    fn get_view_projection_matrix(&self) -> Vec<[f32; 4]> {
        let view_matrix = cgmath::Matrix4::look_at_rh(
            self.position.to_point3(),
            (self.position + self.forward).to_point3(),
            Vector3::unit_y(),
        );

        let projection_matrix = cgmath::perspective(
            cgmath::Deg(45.0), 
            self.aspect_ratio, 
            0.1,
            1000.0
        );

        let mat = OPENGL_TO_WGPU_MATRIX * projection_matrix * view_matrix; 
        let mx_ref : [[f32; 4]; 4] = mat.into();
        mx_ref.to_vec()
    }

    fn as_uniform_buffer(&self, device : &wgpu::Device) -> UniformBuffer {
        let mut camera_data = self.get_view_projection_matrix();
        camera_data.push([self.position.x, self.position.y, self.position.z, 0.0]);
        
        let buffer_size = std::mem::size_of::<[f32; 4]>() * camera_data.len();
        UniformBuffer::new(device, &camera_data, buffer_size as u64)
    }
}