pub struct ProjectionMatrix {
    vfov_rad: f32,
    aspect_ratio: f32,
    z_near: f32,
    z_far: f32
}

impl ProjectionMatrix {
    pub fn new(vfov_rad: f32, aspect_ratio: f32,
               z_near: f32, z_far: f32) -> Self {

        Self {
            vfov_rad,
            aspect_ratio,
            z_near,
            z_far
        }

    }

    pub fn p_inv(&self) -> [[f32; 4]; 4] {
        let h = (self.vfov_rad / 2.0).tan();
        let w = 1.0 / (h * self.aspect_ratio);
        let r = self.z_far / (self.z_far - self.z_near);
        // let p = Mat4::from_cols(
        //     Vec4::new(w, 0.0, 0.0, 0.0),
        //     Vec4::new(0.0, h, 0.0, 0.0),
        //     Vec4::new(0.0, 0.0, r, 1.0),
        //     Vec4::new(0.0, 0.0, -r * z_near, 0.0)
        // );

        let p_inv = [
            [1.0 / w, 0.0, 0.0, 0.0],
            [0.0, 1.0 / h, 0.0, 0.0],
            [0.0, 0.0, 0.0, -1.0 / (r * self.z_near)],
            [0.0, 0.0, 1.0, 1.0 / self.z_near]
        ];

        p_inv
    }
}