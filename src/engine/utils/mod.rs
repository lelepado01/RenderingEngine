use winit::event_loop::ControlFlow;

pub mod vector_extensions;

pub fn get_control_flow_status() -> ControlFlow {
    ControlFlow::Poll
}