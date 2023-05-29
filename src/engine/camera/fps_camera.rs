
use cgmath::{Vector3, InnerSpace};
use winit::event::VirtualKeyCode;
use crate::engine::{engine::EngineData, buffers::uniform_buffer::UniformBuffer, utils::vector_extensions::ToPoint3};
use super::{OPENGL_TO_WGPU_MATRIX, Camera};

pub const SENSITIVITY:f32 = 0.03; 
const SPEED : f32 = 50.0; 

pub struct FpsCamera {
    pub position : Vector3<f32>,
    pub forward : Vector3<f32>,
    pub momentum : Vector3<f32>,
    aspect_ratio : f32,

    yaw : f32,
    pitch : f32,
}

#[allow(dead_code)]
impl FpsCamera {

    pub fn new(start_pos : [f32; 3], aspect_ratio : f32) -> Self {
        Self {
            position : Vector3::from(start_pos),
            forward : Vector3::new(0.0, 0.0, -1.0),
            momentum : Vector3::new(0.0, 0.0, 0.0),

            yaw : 0.0,
            pitch : 0.0,
            aspect_ratio,
        }
    }

    pub fn update(&mut self, delta_time : f32, engine : &EngineData) {

        for keycode in engine.get_keys_pressed() {
            self.handle_input(*keycode);
        }

        self.position += self.momentum * delta_time;

        self.update_position(self.position); 
        self.update_aspect_ratio(engine);      
    }

    pub fn reset_momentum(&mut self) {
        self.momentum = Vector3::new(0.0, 0.0, 0.0); 
    }

    fn update_momentum(&mut self, direction : Vector3<f32>) {
        self.momentum += direction * SPEED;
        self.momentum = self.momentum.normalize() * SPEED;
    }

    fn handle_input(&mut self, keycode : VirtualKeyCode) {
        let forward = Vector3::new(self.forward.x, 0.0, self.forward.z).normalize();
        match keycode {
            VirtualKeyCode::W => { self.update_momentum(forward); }
            VirtualKeyCode::S => { self.update_momentum(-forward); }
            VirtualKeyCode::A => { self.update_momentum(-forward.cross(Vector3::unit_y())); }
            VirtualKeyCode::D => { self.update_momentum(forward.cross(Vector3::unit_y())); }
            VirtualKeyCode::Space => { self.update_momentum(Vector3::unit_y()); }
            VirtualKeyCode::LShift => { self.update_momentum(-Vector3::unit_y()); }
            _ => {}
        }
    }

    pub fn update_rotation(&mut self, x : f32, y : f32) {
        self.yaw += x * SENSITIVITY * 0.05;
        self.pitch -= y * SENSITIVITY * 0.05;

        self.forward = Vector3::new(
            self.pitch.cos() * self.yaw.cos(),
            self.pitch.sin(),
            self.pitch.cos() * self.yaw.sin(),
        );
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

    pub fn update_position(&mut self, player_pos : Vector3<f32>){
        self.position = player_pos;        
    }

    pub fn update_aspect_ratio(&mut self, engine : &EngineData) {
        let window_size = engine.get_window_size();
        self.aspect_ratio = window_size.0 as f32 / window_size.1 as f32;
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
        return mx_ref.to_vec();
    }

    fn as_uniform_buffer(&self, device : &wgpu::Device) -> UniformBuffer {
        let mut camera_data = self.get_view_projection_matrix();
        camera_data.push([self.position.x, self.position.y, self.position.z, 0.0]);
        
        let buffer_size = std::mem::size_of::<[f32; 4]>() * camera_data.len();
        UniformBuffer::new(&device, &camera_data, buffer_size as u64)
    }

    fn get_position(&self) -> [f32; 3] {
        self.position.into()
    }
}
