mod app;
mod path_tracer;
mod compute_shader;

use common_code::bvh;
use common_code::camera::Camera;
use common_code::camera_controller::CameraController;
use common_code::gpu_buffer;
use common_code::gpu_structs;
use common_code::gui;
use common_code::material;
use common_code::parameters;
use common_code::parameters::{RenderParameters, SamplingParameters};
use common_code::scene;
use common_code::sphere;
use glam::Vec3;

use crate::app::App;
use common_code::scene::Scene;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() -> Result<(), EventLoopError> {
    env_logger::init();

    let scene = Scene::new();
    let camera = Camera::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0));
    let camera_controller
        = CameraController::new(90.0,
                                0.0,
                                3.4,
                                0.1,
                                100.0,
                                4.0,
                                0.4);
    let screen_size = (1920, 1080);
    let sampling_parameters = SamplingParameters::new(1,
                                                      50,
                                                      1,
                                                      10);
    let render_parameters = RenderParameters::new(camera, sampling_parameters, screen_size);

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(scene, render_parameters, camera_controller);
    event_loop.run_app(&mut app)
}
