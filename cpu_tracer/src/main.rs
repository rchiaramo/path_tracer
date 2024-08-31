mod app;
mod path_tracer;
mod scene;
mod camera;
mod material;
mod sphere;
mod compute_shader;
mod bvh_node;
mod gui;
mod query_gpu;
mod parameters;
mod gpu_structs;
mod gpu_buffer;

use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::app::App;

fn main() -> Result<(), EventLoopError> {
    env_logger::init();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app)
}
