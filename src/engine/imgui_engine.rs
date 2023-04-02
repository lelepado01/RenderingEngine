use std::time::{Duration};

use imgui::{FontSource};
use imgui_wgpu::{Renderer, RendererConfig};
use wgpu::{TextureView, Device, SurfaceConfiguration};
use winit::window::{Window};

pub struct ImguiEngine{
    pub imgui_context: imgui::Context,
    pub platform: imgui_winit_support::WinitPlatform,
    pub renderer : Renderer,
}

impl ImguiEngine {
    pub fn init(window : &Window, device : &Device, queue : &wgpu::Queue, surface_desc : &SurfaceConfiguration) -> Self {
        let hidpi_factor = window.scale_factor();
        
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );
        imgui.set_ini_filename(None);

        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let renderer_config = RendererConfig {
            texture_format: surface_desc.format,
            ..Default::default()
        };
    
        let renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);    


        ImguiEngine {
            imgui_context: imgui,
            platform,
            renderer,
        }
    }

    pub fn begin_update(&mut self, window: &Window, dt : Duration) {
        self.imgui_context.io_mut().update_delta_time(dt);

        self.platform
            .prepare_frame(self.imgui_context.io_mut(), &window)
            .expect("Failed to prepare frame");

    }
    
    pub fn end_update(&mut self, view : &TextureView, queue : &wgpu::Queue, device : &Device, encoder : &mut wgpu::CommandEncoder) {        
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Do not clear
                    // load: wgpu::LoadOp::Clear(clear_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        self.renderer
            .render(self.imgui_context.render(), &queue, &device, &mut rpass)
            .expect("Rendering failed");

        drop(rpass);
    }

    pub fn handle_event(&mut self, window : &Window, event : &winit::event::Event<()>) {
        self.platform.handle_event(self.imgui_context.io_mut(), &window, event);
    }
}    
