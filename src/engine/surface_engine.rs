use pollster::block_on;
use winit::{dpi::{LogicalPosition, LogicalSize}, event_loop::EventLoop, window::Window};
use crate::engine::consts::{SCREEN_HEIGHT, SCREEN_WIDTH}; 

pub struct SurfaceEngine {
    surface : wgpu::Surface,
    size : winit::dpi::PhysicalSize<u32>,
    adapter : wgpu::Adapter,
    pub window : Window,
    pub frame : std::option::Option<wgpu::SurfaceTexture>,
    view : std::option::Option<wgpu::TextureView>,
}

impl SurfaceEngine {

    pub fn init(event_loop : &EventLoop<()>) -> Self {

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let window = Window::new(event_loop).unwrap();
        window.set_inner_size(LogicalSize {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        });
        window.set_outer_position(LogicalPosition::new(0.0, 0.0));
        window.set_title("Voxel Rendering Engine");

        let size = window.inner_size();
        let surface = unsafe { instance.create_surface(&window) };

        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        SurfaceEngine {
            surface,
            size,
            adapter,
            window,
            frame : None,
            view : None,
        }
    }

    pub fn get_device_and_queue(&self) -> (wgpu::Device, wgpu::Queue) {
        block_on(self.adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("Device Descriptor"),
            features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            limits: wgpu::Limits::default(),
        }, None)).unwrap()
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        (self.size.width, self.size.height)
    }

    pub fn get_surface_desc(&self) -> wgpu::SurfaceConfiguration {
        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: self.size.width,
            height: self.size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        }
    }

    pub fn update_surface(&mut self, device : &wgpu::Device) {
        self.size = self.window.inner_size();

        let surface_desc = self.get_surface_desc();
        self.surface.configure(device, &surface_desc);
    }

    pub fn begin_frame(&mut self) {
        self.frame = self.get_frame();
        self.view = Some(self.frame
            .as_ref()
            .expect("View is not initialized, call begin_frame() first")
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default()));
    }

    pub fn get_frame(&self) -> std::option::Option<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
            Ok(frame) => Some(frame),
            Err(e) => {
                eprintln!("dropped frame: {:?}", e);
                None
            }
        }
    }

    pub fn end_frame(&mut self) {
        self.frame.take().expect("View is not initialized, call begin_frame() first").present();
    }

    pub fn get_view(&self) -> &wgpu::TextureView {
        self.view.as_ref().expect("View is not initialized, call begin_frame() first")
    }
}