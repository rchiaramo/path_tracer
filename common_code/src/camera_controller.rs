use glam::Vec4;
use crate::camera::Camera;

#[derive(Copy, Clone, PartialEq)]
pub struct CameraController {
    camera: Camera,
    vfov_rad: f32,
    defocus_angle_rad: f32,
    focus_distance: f32,
    z_near: f32,
    z_far: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    speed: f32,
    sensitivity: f32
}


impl CameraController {
    pub fn new(camera: Camera, vfov: f32, defocus_angle: f32, focus_distance: f32,
               z_near:f32, z_far: f32, speed: f32, sensitivity: f32) -> Self {
        Self {
            camera,
            vfov_rad: vfov.to_radians(),
            defocus_angle_rad: defocus_angle.to_radians(),
            focus_distance,
            z_near,
            z_far,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            speed,
            sensitivity
        }
    }

    pub fn vfov_rad(&self) -> f32 {
        self.vfov_rad
    }
    pub fn set_vfov(&mut self, vfov:f32) { self.vfov_rad = vfov.to_radians() }

    pub fn dof(&self) -> (f32, f32) {
        (self.defocus_angle_rad, self.focus_distance)
    }
    pub fn set_defocus_angle(&mut self, da:f32) { self.defocus_angle_rad = da.to_radians() }
    pub fn set_focus_distance(&mut self, fd:f32) { self.focus_distance = fd }

    pub fn get_clip_planes(&self) -> (f32, f32) { (self.z_near, self.z_far) }

    pub fn get_GPU_camera(&self) -> GPUCamera {
        GPUCamera::new(&self.camera, self.defocus_angle_rad, self.focus_distance)
    }

    pub fn get_view_matrix(&self) -> [[f32;4];4] {
        self.camera.view_transform()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GPUCamera {
    camera_position: Vec4,
    pitch: f32,
    yaw: f32,
    defocus_radius: f32,
    focus_distance: f32,
}
unsafe impl bytemuck::Pod for GPUCamera {}
unsafe impl bytemuck::Zeroable for GPUCamera {}

impl GPUCamera {
    pub fn new(camera: &Camera, defocus_angle_rad: f32, focus_distance: f32) -> GPUCamera {
        let defocus_radius = focus_distance * (0.5 * defocus_angle_rad).tan();
        let (camera_position, pitch, yaw) = camera.get_camera();

        GPUCamera {
            camera_position: camera_position.extend(0.0),
            pitch,
            yaw,
            defocus_radius,
            focus_distance,
        }
    }

    pub fn position(&self) -> Vec4 { self.camera_position }
    pub fn defocus_radius(&self) -> f32 { self.defocus_radius }
    pub fn focus_distance(&self) -> f32 { self.focus_distance }
}