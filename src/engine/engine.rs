use winit::event::Event;
use winit::event_loop::EventLoop;

use crate::engine::surface_engine;
use crate::engine::imgui_engine;

use super::builders::texture_builder;
use super::env::time::TimeUtils;

pub struct EngineData {
    pub surface_engine : surface_engine::SurfaceEngine,
    device : wgpu::Device,
    queue : wgpu::Queue,
    pub imgui_engine : imgui_engine::ImguiEngine,
    pub clock : TimeUtils,
    mouse_position : (f32, f32),
    keys_pressed : Vec<winit::event::VirtualKeyCode>,
    pub depth_texture : wgpu::TextureView,
}

impl EngineData {
    pub fn new(event_loop : &EventLoop<()>) -> Self {
        let mut surface_engine = surface_engine::SurfaceEngine::init(&event_loop);
        let (device, queue) = surface_engine.get_device_and_queue();
        surface_engine.update_surface(&device); 

        let imgui_engine = imgui_engine::ImguiEngine::init(&surface_engine.window, &device, &queue, &surface_engine.get_surface_desc());

        let clock = TimeUtils::new();

        let config = surface_engine.get_surface_desc();     
        let depth_texture = texture_builder::TextureBuilder::new("", texture_builder::TextureType::TextureDepth)
            .set_dimensions(2)
            .set_extent(config.width, config.height, 1)
            .set_format(wgpu::TextureFormat::Depth32Float)
            .set_usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device, &queue);

        Self {
            surface_engine,
            device,
            queue,
            imgui_engine,
            clock,
            mouse_position : (0.0, 0.0),
            keys_pressed : Vec::new(),
            depth_texture,
        }
    }

    pub fn get_encoder(&self) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        })
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        self.surface_engine.get_window_size()
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn get_mouse_position(&self) -> (f32, f32){
        self.mouse_position
    }

    pub fn get_key_pressed(&self, keycode : winit::event::VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&keycode)
    }

    pub fn get_keys_pressed(&self) -> &Vec<winit::event::VirtualKeyCode> {
        &self.keys_pressed
    }

    pub fn get_depth_texture(&self) -> &wgpu::TextureView {
        &self.depth_texture
    }

    pub fn delta_time(&self) -> f32 {
        self.clock.frame_duration().as_secs_f32()
    }

    pub fn set_mouse_position(&mut self, (x, y) : (f32, f32)){
        self.mouse_position = (x, y);
    }

    pub fn resize_surface(&mut self){
        self.surface_engine.update_surface(&self.device);

        let config = self.surface_engine.get_surface_desc();     
        let depth_texture = texture_builder::TextureBuilder::new("", texture_builder::TextureType::TextureDepth)
            .set_dimensions(2)
            .set_extent(config.width, config.height, 1)
            .set_format(wgpu::TextureFormat::Depth32Float)
            .set_usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&self.device, &self.queue);
        self.depth_texture = depth_texture;
    }

    pub fn request_redraw(&self){
        self.surface_engine.window.request_redraw();
    }

    pub fn update(&mut self){
        self.clock.update();
        self.imgui_engine.begin_update(&self.surface_engine.window, self.clock.frame_duration());
    }

    pub fn update_key_state(&mut self, keycode : winit::event::VirtualKeyCode, pressed : bool){
        if pressed {
            if !self.keys_pressed.contains(&keycode){
                self.keys_pressed.push(keycode);
            }
        } else {
            if let Some(index) = self.keys_pressed.iter().position(|&k| k == keycode){
                self.keys_pressed.remove(index);
            }
        }
    }

    pub fn end_frame(&mut self, mut encoder : wgpu::CommandEncoder){
        self.imgui_engine.end_update(self.surface_engine.get_view(), &self.queue, &self.device, &mut encoder);
        self.queue.submit(Some(encoder.finish()));
        self.surface_engine.end_frame();
    }

    pub fn handle_ui_event(&mut self, event : &Event<()>){
        self.imgui_engine.handle_event(&self.surface_engine.window, &event); 
    }
}