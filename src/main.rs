use engine::{light::DirectionalLight, utils, voxel_engine::VoxelEngine};
use imgui::*;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

mod engine;
use engine::renderer::EngineData;
use engine::camera::fps_camera::FpsCamera;
fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut engine = EngineData::new(&event_loop);
    
    let mut player = FpsCamera::new(&engine);
    let mut light = DirectionalLight::new(); 

    let mut mesh_engine = VoxelEngine::init(engine.get_device(), &engine.surface_engine.get_surface_desc(), &player, &light);

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
                player.update_rotation(new_x - old_x, new_y - old_y);
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
                player.reset_momentum();
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
                player.update(delta_time, &engine);

                mesh_engine.update(engine.get_device(), &player, &light); 

                let mut encoder = engine.get_encoder();
                let ui = engine.imgui_engine.imgui_context.frame();

                engine.surface_engine.begin_frame();

                mesh_engine.render(engine.surface_engine.get_view(), &engine.depth_texture, &mut encoder, &player);

                ui.window("Utils")
                    .size([400.0, 300.0], Condition::FirstUseEver)
                    .build(||{
                        if ui.button("Quit"){
                            *control_flow = ControlFlow::Exit;
                        }

                        let mut camera_position : [f32; 3] = player.position.into();
                        ui.input_float3("Camera position", &mut camera_position).build();
                        let mut camera_rotation : [f32; 3] = player.forward.into();
                        ui.input_float3("Camera rotation", &mut camera_rotation).build();  

                        player.position = camera_position.into(); 
                        player.forward = camera_rotation.into(); 

                        ui.separator();

                        ui.text(format!("FPS: {:.0}", 1.0/delta_time));

                        ui.separator();

                        let mut light_direction : [f32; 3] = light.direction.into();
                        ui.input_float3("Light Direction", &mut light_direction).build();
                        let mut light_color : [f32; 3] = light.color.into();
                        ui.input_float3("Light Color", &mut light_color).build();  

                        light.direction = light_direction.into(); 
                        light.color = light_color.into(); 

                        ui.separator();
                    }
                );                 
                
                engine.end_frame(encoder);
            }
            _ => (),
        }

        engine.handle_ui_event(&event);
    });
}