use glam::{Vec4};
use crate::parameters::SamplingParameters;
use crate::camera::Camera;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GPUCamera {
    pub camera_position: Vec4,
    pub camera_forwards: Vec4,
    pub camera_right: Vec4,
    pub camera_up: Vec4,
    pub pixel_00: Vec4,
    pub du: Vec4,
    pub dv: Vec4,
    pub defocus_radius: f32,
    pub focus_distance: f32,
    _buffer: [u32; 2]
}
unsafe impl bytemuck::Pod for GPUCamera {}
unsafe impl bytemuck::Zeroable for GPUCamera {}

impl GPUCamera {
    pub fn new(camera: &Camera, image_size: (u32, u32)) -> GPUCamera {
        let focus_distance = camera.focus_distance;
        let defocus_radius = focus_distance * (0.5 * camera.defocus_angle).to_radians().tan();
        let theta = camera.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h * camera.focus_distance;
        let viewport_width: f32 = viewport_height * (image_size.0 as f32 / image_size.1 as f32);

        let viewport_u = viewport_width * camera.right;
        let viewport_v = -viewport_height * camera.up;

        let du = viewport_u / image_size.0 as f32;
        let dv = viewport_v / image_size.1 as f32;

        let upper_left = camera.position + camera.focus_distance * camera.forwards -
            0.5 * (viewport_u + viewport_v);
        let pixel_00 = upper_left + 0.5 * (du + dv);

        GPUCamera {
            camera_position: camera.position.extend(0.0),
            camera_forwards: camera.forwards.extend(0.0),
            camera_right: camera.right.extend(0.0),
            camera_up: camera.up.extend(0.0),
            pixel_00: pixel_00.extend(0.0),
            du: du.extend(0.0),
            dv: dv.extend(0.0),
            defocus_radius,
            focus_distance,
            _buffer: [0u32; 2]
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
    
    pub fn into_array(&self) -> [u32;4]{
        [self.width, self.height, self.frame, self.accumulated_samples]
    }
}