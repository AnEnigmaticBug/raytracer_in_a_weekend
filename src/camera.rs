use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

use crate::primitive::{Ray3, Vec3};

#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "CameraInitOptions", into = "CameraInitOptions")]
pub struct Camera {
    pub pos: Vec3,
    pub lower_left_corner: Vec3,
    hz: Vec3,
    vt: Vec3,
    init_options: CameraInitOptions,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CameraInitOptions {
    pub pos: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub vt_fov: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn get_ray(&self, s: f32, t: f32) -> Ray3 {
        Ray3::new(
            self.pos,
            self.lower_left_corner + s * self.hz + t * self.vt - self.pos,
        )
    }
}

impl From<CameraInitOptions> for Camera {
    fn from(options: CameraInitOptions) -> Self {
        let theta = options.vt_fov * PI / 180.0;

        let half_ht = (theta / 2.0).tan();
        let half_wd = options.aspect * half_ht;

        let w = (options.pos - options.look_at).normalized();
        let u = options.vup.cross(&w).normalized();
        let v = w.cross(&u);

        Camera {
            pos: options.pos,
            lower_left_corner: options.pos - half_wd * u - half_ht * v - w,
            hz: 2.0 * half_wd * u,
            vt: 2.0 * half_ht * v,
            init_options: options,
        }
    }
}

impl From<Camera> for CameraInitOptions {
    fn from(camera: Camera) -> Self {
        camera.init_options
    }
}
