use crate::camera::Camera;
use crate::geometry::Scene;
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

    pub fn color_pixel(&self, scene: &Scene, config: &Config, i: u32, j: u32) -> Vec3 {
        let u = (i as f32) / config.canvas_wd as f32;
        let v = (j as f32) / config.canvas_ht as f32;

        let ray = config.camera.get_ray(u, v);
        self.color(&ray, scene, config)
    }

    fn color(&self, ray: &Ray3, scene: &Scene, config: &Config) -> Vec3 {
        if let Some(hit) = scene.hit(ray, 0.0, f32::MAX) {
            0.5 * hit.normal + Vec3::all(0.5)
        } else {
            let t = 0.5 * (ray.dir.normalized().y + 1.0);
            (1.0 - t) * Vec3::all(1.0) + t * config.sky_color
        }
    }
}
