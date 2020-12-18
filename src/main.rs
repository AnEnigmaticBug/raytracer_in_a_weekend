use std::{fs::File, path::Path};

use raytracer::{
    camera::{Camera, CameraInitOptions},
    geometry::{Geometry, Scene, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    primitive::Vec3,
    ray_tracer::{Config, RayTracer},
};

const WD: u32 = 512;
const HT: u32 = 256;

fn main() {
    let ray_tracer = RayTracer::new();
    let scene = setup_scene();
    let config = Config {
        canvas_wd: WD,
        canvas_ht: HT,
        sky_color: Vec3::new(0.5, 0.7, 1.0),
        camera: Camera::with_options(CameraInitOptions {
            pos: Vec3::new(-2.0, 2.0, 1.0),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vt_fov: 30.0,
            aspect: WD as f32 / HT as f32,
        }),
        num_samples: 16,
        max_reflections: 16,
    };

    let pixels = calc_pixels(ray_tracer, scene, config);
    img_writer("scene.png")
        .write_image_data(&pixels)
        .expect("Couldn't write image data");
}

fn setup_scene() -> Scene {
    Scene {
        items: vec![
            Geometry::Sphere(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Lambertian(Lambertian {
                    albedo: Vec3::new(0.8, 0.3, 0.3),
                }),
            }),
            Geometry::Sphere(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::Lambertian(Lambertian {
                    albedo: Vec3::new(0.8, 0.8, 0.0),
                }),
            }),
            Geometry::Sphere(Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Dielectric(Dielectric { ref_idx: 1.5 }),
            }),
            Geometry::Sphere(Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Metal(Metal {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                    fuzz: 1.0,
                }),
            }),
        ],
    }
}

fn calc_pixels(ray_tracer: RayTracer, scene: Scene, config: Config) -> Vec<u8> {
    (0..HT)
        .rev()
        .flat_map(|j| (0..WD).map(move |i| (i, j)))
        .flat_map(|(i, j)| {
            let color = ray_tracer.color_pixel(&scene, &config, i, j);

            let r = (255.99 * color.x) as u8;
            let g = (255.99 * color.y) as u8;
            let b = (255.99 * color.z) as u8;

            vec![r, g, b]
        })
        .collect()
}

fn img_writer<P: AsRef<Path>>(path: P) -> png::Writer<File> {
    let file = File::create(path).expect("Couldn't create file");

    let mut encoder = png::Encoder::new(file, WD, HT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header().unwrap()
}
