use engine::{utils, env::light::LightData};
use game::tilemap::PositionalTileMap;
use imgui::*;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use crate::engine::entity_data::EntityData;

mod engine;
mod game; 
mod physics;

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut engine = engine::engine::EngineData::new(&event_loop);
    
    let mut camera = engine::camera::fps_camera::FpsCamera::new([0.0, 150.0, 0.0], engine.get_window_size().0 as f32 / engine.get_window_size().1 as f32);

    let light = LightData::new([0.0, 15.0, 0.0]);

    let mut tilemap = PositionalTileMap::new();
    let tilemodels = tilemap.as_model(&engine);

    let entity_data = EntityData::new(vec![light], vec![tilemodels], vec![], vec![]);

    let mut mesh_engine = engine::mesh_engine::MeshEngine::init(&engine.get_device(), &engine.surface_engine.get_surface_desc(), &camera, &entity_data);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = utils::get_control_flow_status();

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
                if engine.get_key_pressed(winit::event::VirtualKeyCode::Q) {
                    *control_flow = ControlFlow::Exit;
                }
                
                engine.update();
                let delta_time = engine.delta_time(); 
                camera.update(delta_time, &engine); 
                // tilemap.update(&camera.position.into());

                let tilemodels = tilemap.as_model(&engine);
                let entity_data = EntityData::new(vec![light], vec![tilemodels], vec![], vec![]);

                mesh_engine.update(&engine.get_device(), &camera, &entity_data); 

                let mut encoder = engine.get_encoder();
                let ui = engine.imgui_engine.imgui_context.frame();

                engine.surface_engine.begin_frame();

                mesh_engine.render(&engine.surface_engine.get_view(), &engine.depth_texture, &mut encoder, &entity_data, &mut engine.engine_stats);

                ui.window("Utils")
                    .size([400.0, 300.0], Condition::FirstUseEver)
                    .build(||{
                        ui.text(format!("FPS: {}", engine.engine_stats.fps));
                        ui.text(format!("Frame time: {} ms", engine.engine_stats.frames_render_time));
                        ui.text(format!("Draw Calls: {}", engine.engine_stats.frames_draw_calls));
                        ui.text(format!("Bytes to GPU: {}", engine.engine_stats.bytes_to_gpu));

                        if ui.button("Quit"){
                            *control_flow = ControlFlow::Exit;
                        }

                        let mut camera_position : [f32; 3] = camera.position.into();
                        ui.input_float3("Camera position", &mut camera_position).build();
                        let mut camera_rotation : [f32; 3] = camera.forward.into();
                        ui.input_float3("Camera rotation", &mut camera_rotation).build();  
                    }
                );                 
                
                engine.end_frame(encoder);
            }
            _ => (),
        }

        engine.handle_ui_event(&event);
    });
}