use std::f32::consts::PI;

use crate::primitive::{Ray3, Vec3};

pub struct Camera {
    pub pos: Vec3,
    pub lower_left_corner: Vec3,
    hz: Vec3,
    vt: Vec3,
}

impl Camera {
    pub fn new(pos: Vec3, vt_fov: f32, aspect: f32) -> Self {
        let theta = vt_fov * PI / 180.0;

        let half_ht = (theta / 2.0).tan();
        let half_wd = aspect * half_ht;

        let u = Vec3::new(1.0, 0.0, 0.0);
        let v = Vec3::new(0.0, 1.0, 0.0);

        let z = Vec3::new(0.0, 0.0, 1.0);

        Camera {
            pos,
            lower_left_corner: pos - half_wd * u - half_ht * v - z,
            hz: 2.0 * half_wd * u,
            vt: 2.0 * half_ht * v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray3 {
        Ray3::new(
            self.pos,
            self.lower_left_corner + s * self.hz + t * self.vt - self.pos,
        )
    }
}
