use glam::{Vec4};
use crate::parameters::SamplingParameters;
use crate::Camera;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GPUCamera {
    camera_position: Vec4,
    camera_forwards: Vec4,
    camera_right: Vec4,
    camera_up: Vec4,
    defocus_radius: f32,
    focus_distance: f32,
    _buffer: [u32; 6]
}
unsafe impl bytemuck::Pod for GPUCamera {}
unsafe impl bytemuck::Zeroable for GPUCamera {}

impl GPUCamera {
    pub fn new(camera: &Camera, image_size: (u32, u32)) -> GPUCamera {
        let focus_distance = camera.focus_distance;
        let defocus_radius = focus_distance * (0.5 * camera.defocus_angle).to_radians().tan();

        GPUCamera {
            camera_position: camera.position.extend(0.0),
            camera_forwards: camera.forwards.extend(0.0),
            camera_right: camera.right.extend(0.0),
            camera_up: camera.up.extend(0.0),
            defocus_radius,
            focus_distance,
            _buffer: [0u32; 6]
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUSamplingParameters {
    samples_per_frame: u32,
    num_bounces: u32,
    clear_image_buffer: u32,
    _buffer: u32,
}

// right now this is silly, but later when we add fields to this struct,
// we may have to do some padding for GPU
impl GPUSamplingParameters {
    pub fn get_gpu_sampling_params(sampling_parameters: &SamplingParameters)
                                   -> GPUSamplingParameters
    {
        GPUSamplingParameters {
            samples_per_frame: sampling_parameters.samples_per_frame,
            num_bounces: sampling_parameters.num_bounces,
            clear_image_buffer: sampling_parameters.clear_image_buffer,
            _buffer: 0u32
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GPUFrameBuffer {
    width: u32,
    height: u32,
    frame: u32,
    accumulated_samples: u32
}

impl GPUFrameBuffer {
    pub fn new(width: u32, height: u32, frame: u32, accumulated_samples: u32) -> Self {
        Self {
            width,
            height,
            frame,
            accumulated_samples
        }
    }
}