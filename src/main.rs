// #![allow(dead_code)]

use engine::{engine_utils, env::{self, light::LightData}};
use imgui::*;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod engine;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut engine = engine::engine::EngineData::new(&event_loop);
    let window_size = engine.get_window_size();

    let mut camera = env::camera::Camera::new(window_size.0 as f32 / window_size.1 as f32);
    let mut light = LightData::new();
    let mut light2 = LightData::new();
    light2.position = [10.0, 0.0, 0.0];

    let mut mesh_engine = engine::mesh_engine::MeshEngine::init(&engine.surface_engine.get_surface_desc(), &engine.get_device(), &engine.get_queue(), &camera, &light, &light2);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = engine_utils::get_control_flow_status();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { 
                    position, 
                    ..
                }, ..
            } => {
                let (new_x, new_y) : (f32, f32) = position.into();
                let (old_x, old_y) = engine.get_mouse_position();                
                engine.set_mouse_position(position.into());
                camera.update_rotation(new_x - old_x, new_y - old_y);
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => { engine.resize_surface(); }
            | Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => { engine.update_key_state(keycode, true); }
            | Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state: ElementState::Released,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                engine.update_key_state(keycode, false); 
                camera.reset_momentum();
            }
            | Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => { *control_flow = ControlFlow::Exit; }
            Event::MainEventsCleared => engine.request_redraw(),
            Event::RedrawEventsCleared => {
                let delta_time = engine.delta_time(); 
                camera.update(delta_time, &engine);
                engine.update();

                mesh_engine.update(&camera, &light, &engine.get_device()); 

                let mut encoder = engine.get_encoder();
                let ui = engine.imgui_engine.imgui_context.frame();

                engine.surface_engine.begin_frame();

                mesh_engine.render(&engine.surface_engine.get_view(), &engine.depth_texture, &mut encoder);

                ui.window("Utils")
                    .size([400.0, 300.0], Condition::FirstUseEver)
                    .build(||{
                        ui.text(format!("FPS: {}", engine.clock.fps()));
                        ui.text(format!("Frame time: {} ms", engine.clock.frame_duration().as_millis()));

                        if ui.button("Quit") {
                            *control_flow = ControlFlow::Exit;
                        }

                        let mut camera_position : [f32; 3] = camera.position.into();
                        ui.input_float3("Camera position", &mut camera_position).build();
                        let mut camera_rotation : [f32; 3] = camera.forward.into();
                        ui.input_float3("Camera rotation", &mut camera_rotation).build();  

                        let mut light_position : [f32; 3] = light.position; 
                        ui.slider("Light position x", -10.0, 10.0, &mut light_position[0]); 
                        ui.slider("Light position y", -10.0, 10.0, &mut light_position[1]);
                        ui.slider("Light position z", -10.0, 10.0, &mut light_position[2]);
                        light.position = light_position; 
                    }
                );                 
                
                engine.end_frame(encoder);
            }
            _ => (),
        }

        engine.handle_ui_event(&event);
    });
}