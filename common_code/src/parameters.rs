use crate::camera::Camera;
use crate::gpu_structs::{GPUFrameBuffer, GPUSamplingParameters};

#[derive(Copy, Clone, PartialEq)]
pub struct SamplingParameters {
    pub samples_per_frame: u32,
    pub num_bounces: u32,
    pub clear_image_buffer: u32,
}

impl Default for SamplingParameters {
    fn default() -> Self {

        Self {
            samples_per_frame: 5_u32,
            num_bounces: 50_u32,
            clear_image_buffer: 0_u32,
        }
    }
}

impl SamplingParameters {
    pub fn new(samples_per_frame: u32, num_bounces: u32, clear_image_buffer: u32) -> Self {
        Self {
            samples_per_frame,
            num_bounces,
            clear_image_buffer
        }
    }
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct RenderParameters {
    camera: Camera,
    sampling_parameters: SamplingParameters,
    viewport_size: (u32, u32)
}

impl RenderParameters {
    pub fn new(camera: Camera, sampling_parameters: SamplingParameters, viewport_size: (u32, u32)) -> Self {
        Self {
            camera,
            sampling_parameters,
            viewport_size,
        }
    }

    pub fn set_viewport(&mut self, size: (u32, u32)) {
        self.viewport_size = size;
    }

    pub fn get_viewport(&self) -> (u32, u32) {
        self.viewport_size
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn sampling_parameters(&self) -> &SamplingParameters { &self.sampling_parameters }

}

pub struct RenderProgress {
    frame: u32,
    samples_per_pixel: u32,
    accumulated_samples: u32,
}

impl Default for RenderProgress {
    fn default() -> Self {
        Self {
            frame: 0,
            samples_per_pixel: 1000,
            accumulated_samples: 0
        }
    }
}

impl RenderProgress {
    pub fn new(spp: u32) -> Self {
        Self {
            frame: 0,
            samples_per_pixel: spp,
            accumulated_samples: 0
        }
    }

    pub fn reset(&mut self) {
        self.accumulated_samples = 0;
    }

    pub fn get_next_frame(&mut self, rp: &mut RenderParameters) -> GPUFrameBuffer {
        // if accumulated samples is 0, there's been something that triggered a reset
        let current_progress = self.accumulated_samples;
        let delta_samples = rp.sampling_parameters.samples_per_frame;
        let updated_progress = current_progress + delta_samples;
        let (width, height) = rp.get_viewport();
        let mut frame = 0;
        let mut accumulated_samples = 0;

        if self.accumulated_samples == 0 {
            rp.sampling_parameters = SamplingParameters::new(
                rp.sampling_parameters.samples_per_frame,
                rp.sampling_parameters.num_bounces,
                1
            );
            frame = 1;
            self.frame = 1;
            accumulated_samples = delta_samples;
            self.accumulated_samples = accumulated_samples;
        } else if updated_progress > self.samples_per_pixel {
            rp.sampling_parameters = SamplingParameters::new(
              0,
              rp.sampling_parameters.num_bounces,
              0
            );
            self.frame += 1;
            frame = self.frame;
            accumulated_samples = current_progress;
        } else {
            rp.sampling_parameters = SamplingParameters::new(
              rp.sampling_parameters.samples_per_frame,
              rp.sampling_parameters.num_bounces,
              0
            );
            self.frame += 1;
            frame = self.frame;
            self.accumulated_samples = updated_progress;
            accumulated_samples = updated_progress;
        }

        GPUFrameBuffer::new(width, height, frame, accumulated_samples)
    }

    pub fn progress(&self) -> f32 {
        self.accumulated_samples as f32 / self.samples_per_pixel as f32
    }
}



