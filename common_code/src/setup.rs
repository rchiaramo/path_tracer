use glam::Vec3;

#[derive(Clone)]
pub struct Setup {
    pub window_size: (u32, u32),
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vfov: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    pub speed: f32,
    pub sensitivity: f32
}