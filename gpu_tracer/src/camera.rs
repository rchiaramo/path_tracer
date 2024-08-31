use glam::{Vec3};

#[derive(Copy, Clone, PartialEq)]
pub struct Camera {
    pub position: Vec3,
    pub forwards: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub vfov: f32,
    pub defocus_angle: f32,
    pub focus_distance: f32,
}

impl Default for Camera {
    fn default() -> Self {
        let lookAt = Vec3::new(0.0, 0.0, -1.0);
        let lookFrom = Vec3::new(0.0, 0.0, 1.0);
        let forwards = (lookAt - lookFrom).normalize();
        let right = forwards.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(forwards);
        let vfov = 90.0;
        let defocus_angle = 0.0_f32;
        let focus_distance = 3.4_f32;

        Self {
            position: lookFrom,
            forwards,
            right,
            up,
            vfov,
            defocus_angle,
            focus_distance
        }
    }
}

impl Camera {
    pub fn new(lookAt: Vec3, lookFrom: Vec3, vfov: f32, defocus_angle: f32, focus_distance: f32) -> Self {
        let forwards = (lookAt - lookFrom).normalize();
        let right = forwards.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(forwards);

        Self {
            position: lookFrom,
            forwards,
            right,
            up,
            vfov,
            defocus_angle,
            focus_distance
        }
    }

    pub fn book_one_final_camera() -> Self {
        let lookAt = Vec3::new(0.0, 0.0, 0.0);
        let lookFrom = Vec3::new(13.0, 2.0, 3.0);
        let vfov = 20.0f32;
        let defocus_angle = 0.6_f32;
        let focus_distance = 10.0_f32;
        Self::new(lookAt, lookFrom, vfov, defocus_angle, focus_distance)
    }

    pub fn projection_transform(& self,
                            aspect_ratio: f32,
                            z_near: f32,
                            z_far: f32) -> [[f32;4];4] {

        let v_fov = self.vfov.to_radians();
        let h = 1.0 / (v_fov / 2.0).tan();
        let w = h / aspect_ratio;
        let r = z_far / (z_far - z_near);
        // let p = Mat4::from_cols(
        //     Vec4::new(w, 0.0, 0.0, 0.0),
        //     Vec4::new(0.0, h, 0.0, 0.0),
        //     Vec4::new(0.0, 0.0, r, 1.0),
        //     Vec4::new(0.0, 0.0, -r * z_near, 0.0)
        // );

        let p_inv = [
            [1.0 / w, 0.0, 0.0, 0.0],
            [0.0, 1.0 / h, 0.0, 0.0],
            [0.0, 0.0, 0.0, -1.0 / (r * z_near)],
            [0.0, 0.0, 1.0, 1.0 / z_near]
        ];

        p_inv
    }

    pub fn view_transform(& self) -> [[f32; 4]; 4]
    {
        // look-at transformation with x-axis flipped to account for
        // rh world coordinates but lh camera coordinates
        let dir = self.forwards;
        let right = self.right;
        let new_up = self.up;
        let center = self.position;

        let world_from_camera = [
            [right.x, right.y, right.z, 0.0],
            [new_up.x, new_up.y, new_up.z, 0.0],
            [dir.x, dir.y, dir.z, 0.0],
            [center.x, center.y, center.z, 1.0]
        ];

        // if wfc is of form T*R, then inv is inv(T)*inv(T), which is why we have the dot
        // product now in the fourth column
        // let camera_from_world = Mat4::from_cols(
        //     Vec4::new(-right.x, new_up.x, dir.x, 0.0),
        //     Vec4::new(-right.y, new_up.y, dir.y, 0.0),
        //     Vec4::new(-right.z, new_up.z, dir.z, 0.0),
        //     Vec4::new(center.dot(right), -center.dot(new_up), -center.dot(dir), 1.0)
        // );

        world_from_camera
    }
}