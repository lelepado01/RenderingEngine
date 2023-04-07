use std::f32::consts::PI;

use cgmath::{Vector3, InnerSpace};
use winit::event::VirtualKeyCode;

use crate::engine::{engine::EngineData, buffers::uniform_buffer::UniformBuffer, utils::vector_extensions::ToPoint3};

use super::{OPENGL_TO_WGPU_MATRIX};

#[allow(dead_code)]
pub struct FpsCamera {
    pub position : Vector3<f32>,
    pub forward : Vector3<f32>,
    momentum : Vector3<f32>,
    speed : f32,
    aspect_ratio : f32,

    yaw : f32,
    pitch : f32,
}


#[allow(dead_code)]
impl FpsCamera {

    pub fn new(aspect_ratio : f32) -> Self {
        Self {
            position : Vector3::new(0.0, 0.0, 0.0),
            forward : Vector3::new(0.0, 0.0, -1.0),
            momentum : Vector3::new(0.0, 0.0, 0.0),
            speed : 20.0,
            yaw : 0.0,
            pitch : 0.0,
            aspect_ratio,
        }
    }

    pub fn reset_momentum(&mut self) {
        self.momentum = Vector3::new(0.0, 0.0, 0.0);
    }

    pub fn update(&mut self, delta_time : f32, engine : &EngineData) {

        for keycode in engine.get_keys_pressed() {
            self.handle_input(*keycode);
        }

        self.forward = Vector3::new(
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        );

        self.position += self.momentum * delta_time;
        let window_size = engine.get_window_size();
        self.aspect_ratio = window_size.0 as f32 / window_size.1 as f32;
    }

    pub fn update_rotation(&mut self, x : f32, y : f32) {
        self.yaw += x*0.0008;
        self.pitch += -y*0.0008;
        self.pitch = self.pitch.max(-PI / 2.0).min(PI / 2.0);

        self.forward = Vector3::new(
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        );
    }

    fn update_momentum(&mut self, direction : Vector3<f32>) {
        self.momentum += direction * self.speed;
        self.momentum = self.momentum.normalize() * self.speed;
    }

    fn handle_input(&mut self, keycode : VirtualKeyCode) {
        match keycode {
            VirtualKeyCode::W => {
                self.update_momentum(self.forward);
            }
            VirtualKeyCode::S => {
                self.update_momentum(-self.forward);
            }
            VirtualKeyCode::A => {
                self.update_momentum(-self.forward.cross(Vector3::unit_y()));
            }
            VirtualKeyCode::D => {
                self.update_momentum(self.forward.cross(Vector3::unit_y()));
            }
            VirtualKeyCode::Space => {
                self.update_momentum(Vector3::unit_y()); 
            }
            VirtualKeyCode::LShift => {
                self.update_momentum(-Vector3::unit_y()); 
            }
            _ => {}
        }
    }

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