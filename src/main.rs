use rand::{thread_rng, Rng};
use raytracer::{
    camera::{Camera, CameraInitOptions},
    geometry::{Geometry, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    primitive::Vec3,
    ray_tracer::{Config, RayTracer},
    scene::Scene,
};

const WD: u32 = 512;
const HT: u32 = 256;

fn main() {
    let ray_tracer = RayTracer::new();
    let scene = setup_scene();
    let config = Config {
        canvas_wd: WD,
        canvas_ht: HT,
        num_samples: 16,
        max_reflections: 16,
    };

    ray_tracer
        .render_to_file(&scene, &config, "scene.png")
        .expect("Couldn't write image data");
}

fn setup_scene() -> Scene {
    let mut scene = Scene {
        sky_color: Vec3::new(0.5, 0.7, 1.0),
        camera: Camera::with_options(CameraInitOptions {
            pos: Vec3::new(9.0, 2.0, 2.0),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vt_fov: 30.0,
            aspect: WD as f32 / HT as f32,
        }),
        items: Vec::with_capacity(1 + 12 * 12 + 3),
    };

    scene.items.push(Geometry::Sphere(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian(Lambertian {
            albedo: Vec3::all(0.5),
        }),
    }));

    let mut rng = thread_rng();

    for a in -6..6 {
        for b in -6..6 {
            let offset = Vec3::new(rng.gen(), 0.0, rng.gen()) * 0.9;
            let center = Vec3::new(a as f32, 0.2, b as f32) + offset;

            let material_chooser = rng.gen::<f32>();
            let material = if material_chooser < 0.8 {
                Material::Lambertian(Lambertian {
                    albedo: Vec3::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    ),
                })
            } else if material_chooser < 0.95 {
                Material::Metal(Metal {
                    albedo: Vec3::new(
                        0.5 * (rng.gen::<f32>() + 1.0),
                        0.5 * (rng.gen::<f32>() + 1.0),
                        0.5 * (rng.gen::<f32>() + 1.0),
                    ),
                    fuzz: 0.5 * rng.gen::<f32>(),
                })
            } else {
                Material::Dielectric(Dielectric { ref_idx: 1.5 })
            };

            scene.items.push(Geometry::Sphere(Sphere {
                center,
                radius: 0.2,
                material,
            }));
        }
    }

    scene.items.push(Geometry::Sphere(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    }));

    scene.items.push(Geometry::Sphere(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(Dielectric { ref_idx: 1.5 }),
    }));

    scene.items.push(Geometry::Sphere(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    scene
}
