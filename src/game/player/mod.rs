use cgmath::{Vector3, InnerSpace};
use winit::event::VirtualKeyCode;

use crate::engine::{
    camera::fps_camera::FpsCamera, engine::EngineData
};

pub struct Player {
    // pub camera : ThirdPersonCamera,
    pub camera : FpsCamera,
    pub position : Vector3<f32>,
    momentum : Vector3<f32>, 
}

const SPEED : f32 = 50.0; 

impl Player {
    pub fn new(engine : &EngineData) -> Self {
        let window_size = engine.get_window_size();
        // let camera = ThirdPersonCamera::new([0.0, 10.0, 0.0], window_size.0 as f32 / window_size.1 as f32);

        let camera = FpsCamera::new([0.0, 10.0, 0.0], window_size.0 as f32 / window_size.1 as f32);

        Player {
            position : camera.position,
            camera,
            momentum: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self, delta_time : f32, engine : &EngineData) {

        for keycode in engine.get_keys_pressed() {
            self.handle_input(*keycode);
        }

        self.position += self.momentum * delta_time;

        self.camera.update_position(self.position); 
        self.camera.update_aspect_ratio(engine);      
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
}