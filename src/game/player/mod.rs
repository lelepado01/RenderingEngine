use cgmath::{Vector3, InnerSpace, SquareMatrix};
use winit::event::VirtualKeyCode;

use crate::engine::{models::standard_model::{self, StandardModel}, engine::EngineData, buffers::uniform_buffer::UniformBuffer, camera::third_person_camera::ThirdPersonCamera};

mod aesthetics;

pub struct Player {
    pub model : standard_model::StandardModel,
    pub camera : ThirdPersonCamera,

    pub position : Vector3<f32>,
    momentum : Vector3<f32>, 

    pub aesthetics : aesthetics::PlayerAestheticsParams,
}

const SPEED : f32 = 50.0; 

const IDENTITY_MATRIX : [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

impl Player {
    pub fn new(engine : &EngineData) -> Self {
        let model = "assets/clown_fish.obj"; 
        let mut fish = StandardModel::new(
            &engine.get_device(), 
            model, 
        ).expect("Failed to create OBJ model");

        let window_size = engine.get_window_size();
        let camera = ThirdPersonCamera::new([0.0, 30.0, 0.0], window_size.0 as f32 / window_size.1 as f32);

        let size = std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress * IDENTITY_MATRIX.len() as wgpu::BufferAddress;
        let buffer = UniformBuffer::new(
            &engine.get_device(),
            &IDENTITY_MATRIX.into(), 
            size,
        );
        fish.set_uniform_buffer(buffer);

        Player {
            model : fish,
            camera,

            position : Vector3::new(0.0, 0.0, 0.0),
            momentum: Vector3::new(0.0, 0.0, 0.0),

            aesthetics : aesthetics::PlayerAestheticsParams::new(model.to_string()),
        }
    }

    pub fn update(&mut self, delta_time : f32, engine : &EngineData) {

        for keycode in engine.get_keys_pressed() {
            self.handle_input(*keycode);
        }

        self.position += self.momentum * delta_time;
        self.aesthetics.update(delta_time, engine, self.momentum);

        self.camera.update_position(self.position); 
        self.camera.update_aspect_ratio(engine);      

        let model_matrix = self.get_model_matrix();
        let size = std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress * model_matrix.len() as wgpu::BufferAddress;
        self.model.update_uniform_buffer(engine.get_device(), &model_matrix, size);
    }

    pub fn reset_momentum(&mut self) {
        self.momentum = Vector3::new(0.0, 0.0, 0.0); 
    }

    fn update_momentum(&mut self, direction : Vector3<f32>) {
        self.momentum += direction * SPEED;
        self.momentum = self.momentum.normalize() * SPEED;
    }

    fn handle_input(&mut self, keycode : VirtualKeyCode) {
        let forward = Vector3::new(self.camera.forward.x, 0.0, self.camera.forward.z).normalize();
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

    fn get_model_matrix(&mut self) -> Vec<[f32; 4]> {
        let mut model_matrix = cgmath::Matrix4::identity();
        model_matrix = model_matrix * cgmath::Matrix4::from_translation(
            self.aesthetics.get_aesthetic_position(self.position)
        );
        model_matrix = model_matrix * cgmath::Matrix4::from_scale(3.0);
        model_matrix = model_matrix * cgmath::Matrix4::from_axis_angle(
            cgmath::Vector3::unit_y(), 
            self.aesthetics.get_aesthetic_angle()
        );
        
        let mut v = Vec::new(); 
        let modmat : [[f32; 4]; 4] = model_matrix.into();
        for m in modmat {
            v.push(m);
        }
        v
    }
}