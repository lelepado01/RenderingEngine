use crate::engine::{models::{model, loading}, engine::EngineData, env::camera, buffers::uniform_buffer::UniformBuffer, utils::array_extentions::ToArray4};

pub struct Player {
    pub model : model::Model,
    pub camera : camera::Camera,
    pub position : [f32; 3],
}

impl Player {
    pub fn new(engine : &EngineData) -> Self {
        let mut fish = loading::load_model(
            &engine.get_device(), 
            &engine.get_queue(),
            "assets/seahorse.obj", 
        ).expect("Failed to create OBJ model");

        let window_size = engine.get_window_size();
        let camera = camera::Camera::new(window_size.0 as f32 / window_size.1 as f32);

        let size = std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress;
        let buffer = UniformBuffer::new(
            &engine.get_device(),
            &vec![[0.0, 0.0, 0.0, 0.0]], 
            size,
        );
        fish.set_uniform_buffer(buffer);

        Player {
            model : fish,
            position : [0.0, 0.0, 0.0],
            camera
        }
    }

    pub fn update(&mut self, delta_time : f32, engine : &EngineData) {
        self.camera.update(delta_time, engine);
        self.update_model_position();

        let size = std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress;
        self.model.update_uniform_buffer(engine.get_device(), &vec![self.position.to_arr4()], size); 

    }

    fn update_model_position(&mut self) {
        self.position = (self.camera.position + self.camera.forward * 10.0).into(); 
    }
}