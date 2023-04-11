use engine::{utils, env::light::LightData, models::{vertices::instance_data::PositionInstanceData, instanced_model}};
use imgui::*;
use winit::{
    event::{ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use crate::engine::entity_data::EntityData;

mod engine;
mod game; 

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut engine = engine::engine::EngineData::new(&event_loop);
    
    let mut player = game::player::Player::new(&engine);

    let light = LightData::new([0.0, 15.0, 0.0]);

    let mut poss = Vec::<[f32; 5]>::new();
    
    for i in 0..300 {
        for j in 0..300 {
            let height = (i as f32 * 0.2).sin() * 3.0 * (j as f32 * 0.1).cos() * 3.0;
            let mat_id : f32 = (i % 3) as f32;
            poss.push([2.0 * i as f32, height, 2.0* j as f32, 1.0, mat_id]);
        }
    }
    let instances : Vec<PositionInstanceData> = poss.into_iter().map(|x| PositionInstanceData { position: [x[0], x[1], x[2], x[3]], material_index: [x[4], 0.0, 0.0, 0.0] }).collect();
    let mut model = instanced_model::InstancedModel::new(
        &engine.get_device(), 
        "assets/cube.obj", 
        instances,
    ); 

    let entity_data = EntityData::new(vec![light], vec![&model], vec![&player.model]);

    let mut mesh_engine = engine::mesh_engine::MeshEngine::init(&engine.get_device(), &engine.surface_engine.get_surface_desc(), &player.camera, &entity_data);

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
                player.camera.update_rotation(new_x - old_x, new_y - old_y);
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
                let delta_time = engine.delta_time(); 
                player.update(delta_time, &engine);
                engine.update();

                let mut poss = Vec::<[f32; 5]>::new();
    
                for i in 0..300 {
                    for j in 0..300 {
                        let height = (i as f32 * 0.2 + engine.clock.get_time()).sin() * 3.0 * (j as f32 * 0.1 + engine.clock.get_time() * 0.5).cos() * 3.0;
                        let material_id = (i % 3) as f32;
                        poss.push([2.0 * i as f32, height, 2.0* j as f32, 1.0, material_id]);
                    }
                }
                let instances : Vec<PositionInstanceData> = poss.into_iter().map(|x| PositionInstanceData { position: [x[0], x[1], x[2], x[3]], material_index: [x[4], 0.0, 0.0, 0.0] }).collect();
                model.update_instances(&engine.get_device(), &instances); 
                let entity_data = EntityData::new(vec![light], vec![&model], vec![&player.model]);

                mesh_engine.update(&engine.get_device(), &player.camera, &entity_data); 

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

                        if ui.button("Quit") {
                            *control_flow = ControlFlow::Exit;
                        }

                        let mut camera_position : [f32; 3] = player.camera.position.into();
                        ui.input_float3("Camera position", &mut camera_position).build();
                        let mut camera_rotation : [f32; 3] = player.camera.forward.into();
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