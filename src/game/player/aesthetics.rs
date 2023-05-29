use cgmath::{Rad, Vector3, VectorSpace};

use crate::engine::engine::EngineData;


pub struct PlayerAestheticsParams {
    y_fluctuation : f32,
    old_momentum : Vector3<f32>,

    forward_rotation : Rad<f32>,
}

impl PlayerAestheticsParams {
    pub fn new(file_name : String) -> Self {
        let mut rot = cgmath::Rad(0.0);
        if file_name.contains("seahorse") {
            rot = cgmath::Rad(std::f32::consts::PI / 2.0);
        } 

        Self {
            y_fluctuation : 0.0,
            old_momentum : Vector3::new(0.0, 0.0, 0.0),
            forward_rotation : rot,
        }
    }

    pub fn update(&mut self, delta_time : f32, engine : &EngineData, momentum : Vector3<f32>) {
        self.y_fluctuation = (engine.clock.get_time() * 2.0).sin() * 0.5;
        self.old_momentum = self.old_momentum.lerp(momentum, delta_time * 10.0);
    }

    pub fn get_aesthetic_position(&self, position : Vector3<f32>) -> Vector3<f32> {
        position + Vector3::new(0.0, self.y_fluctuation, 0.0)
    }

    pub fn get_aesthetic_angle(&self) -> Rad<f32> {
        cgmath::Rad(self.old_momentum.x.atan2(self.old_momentum.z)) - self.forward_rotation
    }
}