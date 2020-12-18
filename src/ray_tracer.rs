use rand::Rng;

use crate::camera::Camera;
use crate::geometry::Scene;
use crate::primitive::{Ray3, Vec3};

pub struct RayTracer {}

pub struct Config {
    pub canvas_wd: u32,
    pub canvas_ht: u32,
    pub sky_color: Vec3,
    pub camera: Camera,
    pub num_samples: u8,
    pub max_reflections: u8,
}

impl RayTracer {
    pub fn new() -> Self {
        RayTracer {}
    }

    pub fn color_pixel(&self, scene: &Scene, config: &Config, i: u32, j: u32) -> Vec3 {
        let mut color = Vec3::all(0.0);
        let mut rng = rand::thread_rng();

        for _ in 0..config.num_samples {
            let u = (i as f32 + rng.gen::<f32>()) / config.canvas_wd as f32;
            let v = (j as f32 + rng.gen::<f32>()) / config.canvas_ht as f32;

            let ray = config.camera.get_ray(u, v);
            color = color + self.color(&ray, scene, config, 0);
        }

        let color = color / config.num_samples as f32;
        Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt())
    }

    fn color(&self, ray: &Ray3, scene: &Scene, config: &Config, depth: u8) -> Vec3 {
        if depth >= config.max_reflections {
            return Vec3::all(0.0);
        }

        if let Some(hit) = scene.hit(ray, 0.001, f32::MAX) {
            if let Some(info) = hit.material.interact(ray, &hit) {
                self.color(&info.ray, scene, config, depth + 1) * info.attenuation
            } else {
                Vec3::all(0.0)
            }
        } else {
            let t = 0.5 * (ray.dir.normalized().y + 1.0);
            (1.0 - t) * Vec3::all(1.0) + t * config.sky_color
        }
    }
}
