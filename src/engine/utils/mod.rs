use winit::event_loop::ControlFlow;

pub mod array_math; 
pub mod array_extentions;
pub mod vector_extensions;

pub fn get_control_flow_status() -> ControlFlow {
    if cfg!(feature = "metal-auto-capture") {
        ControlFlow::Exit
    } else {
        ControlFlow::Poll
    }
}