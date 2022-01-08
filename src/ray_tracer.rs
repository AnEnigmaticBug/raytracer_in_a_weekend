use std::fs::File;
use std::path::Path;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

use crate::primitive::{Ray3, Vec3};
use crate::scene::Scene;

pub struct RayTracer {
    pub canvas_wd: u32,
    pub canvas_ht: u32,
    pub num_samples: u8,
    pub max_reflections: u8,
}

impl RayTracer {
    pub fn render_to_file<P: AsRef<Path>>(
        &self,
        scene: &Scene,
        path: P,
    ) -> Result<(), png::EncodingError> {
        let pixels = self.color_scene(&scene);

        let file = File::create(path).expect("Couldn't create file");

        let mut encoder = png::Encoder::new(file, self.canvas_wd, self.canvas_ht);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&pixels)
    }

    pub fn color_scene(&self, scene: &Scene) -> Vec<u8> {
        let bar = ProgressBar::new(self.canvas_ht as u64).with_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%")
                .progress_chars("#>-"),
        );
        bar.set_draw_delta(4);

        (0..self.canvas_ht)
            .into_par_iter()
            .rev()
            .progress_with(bar)
            .flat_map_iter(|j| (0..self.canvas_wd).map(move |i| (i, j)))
            .flat_map_iter(|(i, j)| {
                let color = self.color_pixel(&scene, i, j);

                let r = (255.99 * color.x) as u8;
                let g = (255.99 * color.y) as u8;
                let b = (255.99 * color.z) as u8;

                vec![r, g, b]
            })
            .collect()
    }

    fn color_pixel(&self, scene: &Scene, i: u32, j: u32) -> Vec3 {
        let mut color = Vec3::all(0.0);
        let mut rng = rand::thread_rng();

        for _ in 0..self.num_samples {
            let u = (i as f32 + rng.gen::<f32>()) / self.canvas_wd as f32;
            let v = (j as f32 + rng.gen::<f32>()) / self.canvas_ht as f32;

            let ray = scene.camera.get_ray(u, v);
            color = color + self.color_ray(&ray, scene, 0);
        }

        let color = color / self.num_samples as f32;
        Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt())
    }

    fn color_ray(&self, ray: &Ray3, scene: &Scene, depth: u8) -> Vec3 {
        if depth >= self.max_reflections {
            return Vec3::all(0.0);
        }

        if let Some(hit) = scene.hit(ray, 0.001, f32::MAX) {
            if let Some(info) = hit.material.interact(ray, &hit) {
                self.color_ray(&info.ray, scene, depth + 1) * info.attenuation
            } else {
                Vec3::all(0.0)
            }
        } else {
            let t = 0.5 * (ray.dir.normalized().y + 1.0);
            (1.0 - t) * Vec3::all(1.0) + t * scene.sky_color
        }
    }
}
