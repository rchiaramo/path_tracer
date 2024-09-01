use glam::Vec3;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};
use gpu_tracer::App;
use common_code::setup::Setup;

fn main() -> Result<(), EventLoopError> {
    env_logger::init();
    
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let setup = Setup {
        window_size: (1920, 1080), //3840x2160
        look_from: Vec3::new(0.0, 0.0, 1.0),
        look_at: Vec3::new(0.0, 0.0, -1.0),
        vfov: 90.0,
        z_near: 0.1,
        z_far: 100.0,
        defocus_angle: 0.0,
        focus_distance: 3.4,
        speed: 4.0,
        sensitivity: 0.4
    };
    let mut app = App::new(setup);
    event_loop.run_app(&mut app)
}