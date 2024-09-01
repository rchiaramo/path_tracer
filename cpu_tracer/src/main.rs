mod app;
mod path_tracer;
mod compute_shader;
use common_code::camera;
use common_code::scene;
use common_code::gpu_structs;
use common_code::gpu_buffer;
use common_code::projection_matrix;
use common_code::parameters;
use common_code::bvh;
use common_code::gui;
use common_code::sphere;
use common_code::material;

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
