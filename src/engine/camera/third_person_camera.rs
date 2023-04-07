use std::f32::consts::PI;

use cgmath::{Vector3, InnerSpace};

use crate::engine::{engine::EngineData, buffers::uniform_buffer::UniformBuffer};


pub struct ThirdPersonCamera {
    pub position : Vector3<f32>,
    pub forward : Vector3<f32>,
    aspect_ratio : f32,

    yaw : f32,
    pitch : f32,
}

const DISTANCE_PLAYER_CAMERA : f32 = 15.0; 
const SENSITIVITY:f32 = 0.1; 

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


impl ThirdPersonCamera {

    pub fn new(aspect_ratio : f32) -> Self {
        Self {
            position : Vector3::new(0.0, 10.0, 0.0),
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
        
        self.position.x = DISTANCE_PLAYER_CAMERA * ((self.yaw + 270.0) * PI / 180.0).cos() + player_pos.x;
        self.position.z = DISTANCE_PLAYER_CAMERA * ((self.yaw - 270.0) * PI / 180.0).sin() + player_pos.z;
        self.position.y = DISTANCE_PLAYER_CAMERA * ((self.pitch - 270.0) * PI / 180.0).sin() + player_pos.y;
        
        self.forward = (player_pos - self.position).normalize();
    }

    pub fn update_rotation(&mut self, x : f32, y : f32) {
        self.yaw += x*SENSITIVITY;
        self.pitch += -y*SENSITIVITY;
    }

    pub fn get_view_projection_matrix(&self) -> Vec<[f32; 4]> {
        let view_matrix = cgmath::Matrix4::look_at_rh(
            vector3_to_point3(self.position),
            vector3_to_point3(self.position + self.forward),
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
        return mx_ref.to_vec();
    }

    pub fn get_camera_data(&self) -> Vec<[f32; 4]> {
        let mut data = self.get_view_projection_matrix();
        data.push([self.position.x, self.position.y, self.position.z, 0.0]);
        return data;
    }

    pub fn as_uniform_buffer(&self, device : &wgpu::Device) -> UniformBuffer {
        let camera_data = self.get_camera_data();
        let buffer_size = std::mem::size_of::<[f32; 4]>() * camera_data.len();
        UniformBuffer::new(&device, &camera_data, buffer_size as u64)
    }

}

fn vector3_to_point3(v : Vector3<f32>) -> cgmath::Point3<f32> {
    cgmath::Point3::new(v.x, v.y, v.z)
}