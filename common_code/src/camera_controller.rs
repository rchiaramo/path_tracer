use crate::camera::Camera;

#[derive(Default, Copy, Clone)]
pub struct CameraController {
    vfov_rad: f32,
    defocus_angle_rad: f32,
    focus_distance: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    speed: f32,
    sensitivity: f32
}


impl CameraController {
    pub fn new(vfov: f32, defocus_angle: f32, focus_distance: f32,
               speed: f32, sensitivity: f32) -> Self {
        Self {
            vfov_rad: vfov.to_radians(),
            defocus_angle_rad: defocus_angle.to_radians(),
            focus_distance,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            speed,
            sensitivity
        }
    }

    pub fn vfov_rad(&self) -> f32 {
        self.vfov_rad
    }

    pub fn dof(&self) -> (f32, f32) {
        (self.defocus_angle_rad, self.focus_distance)
    }

    pub fn update_camera(&self, camera: &mut Camera) {

    }
}