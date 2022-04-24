use std::path::Path;

use clap::Args;
use glam::Vec3;
use image::{ColorType, ImageResult};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

use crate::material::Interaction;
use crate::primitive::Ray3;
use crate::scene::{HitInfoAndMaterial, Scene};
use crate::tone_mapper::ToneMapper;

#[derive(Args)]
pub struct RayTracer {
    /// Width of the rendered image
    #[clap(long = "wd", default_value_t = 1440)]
    pub canvas_wd: u32,

    /// Height of the rendered image
    #[clap(long = "ht", default_value_t = 720)]
    pub canvas_ht: u32,

    /// The number of samples taken per pixel. Higher values mean lesser noise.
    #[clap(short = 's', long = "samples", default_value_t = 48)]
    pub num_samples: u16,

    /// The maximum number of reflections per light ray. Most scenes don't need
    /// more than 20 reflections.
    #[clap(short = 'r', long, default_value_t = 16)]
    pub max_reflections: u8,

    /// Tone mapper maps HDR (High Dynamic Range) color values to SDR (Standard
    /// Dynamic Range) color values. Different tone mappers can change the same
    /// HDR inputs into very different outputs.
    #[clap(long = "tone-mapper", arg_enum, default_value = "clamp")]
    pub tone_mapper: ToneMapper,
}

impl RayTracer {
    pub fn render_to_file<P: AsRef<Path>>(&self, scene: &Scene, path: P) -> ImageResult<()> {
        let pixels = self.color_scene(&scene);
        image::save_buffer(
            path,
            &pixels,
            self.canvas_wd,
            self.canvas_ht,
            ColorType::Rgb8,
        )
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
                let color = self.tone_mapper.map(self.color_pixel(&scene, i, j));

                let r = (255.99 * color.x) as u8;
                let g = (255.99 * color.y) as u8;
                let b = (255.99 * color.z) as u8;

                vec![r, g, b]
            })
            .collect()
    }

    fn color_pixel(&self, scene: &Scene, i: u32, j: u32) -> Vec3 {
        let mut color = Vec3::ZERO;
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
            return Vec3::ZERO;
        }

        if let Some(HitInfoAndMaterial(hit_info, material)) = scene.hit(ray, 0.001, f32::MAX) {
            match material.interact(&scene.texture_cache, ray, &hit_info) {
                Interaction::NonTerminal { ray, attenuation } => {
                    self.color_ray(&ray, scene, depth + 1) * attenuation
                }
                Interaction::Terminal { color } => color,
            }
        } else {
            scene.sky_box.color(&scene.texture_cache, ray.dir)
        }
    }
}
