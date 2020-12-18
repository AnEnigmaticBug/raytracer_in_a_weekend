use crate::camera::Camera;
use crate::primitive::{Ray3, Vec3};

pub struct RayTracer {}

pub struct Config {
    pub canvas_wd: u32,
    pub canvas_ht: u32,
    pub sky_color: Vec3,
    pub camera: Camera,
}

impl RayTracer {
    pub fn new() -> Self {
        RayTracer {}
    }

    pub fn color_pixel(&self, config: &Config, i: u32, j: u32) -> Vec3 {
        let u = (i as f32) / config.canvas_wd as f32;
        let v = (j as f32) / config.canvas_ht as f32;

        let ray = config.camera.get_ray(u, v);
        self.color(&ray, config)
    }

    fn color(&self, ray: &Ray3, config: &Config) -> Vec3 {
        let t = 0.5 * (ray.dir.normalized().y + 1.0);
        (1.0 - t) * Vec3::all(1.0) + t * config.sky_color
    }
}
