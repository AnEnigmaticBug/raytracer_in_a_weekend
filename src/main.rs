use raytracer::{
    camera::Camera,
    geometry::{Geometry, Scene, Sphere},
    material::{Lambertian, Material},
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
        camera: Camera::new(Vec3::all(0.0), 90.0, WD as f32 / HT as f32),
        num_samples: 16,
    };

    println!("P3\n{} {}\n255", WD, HT);

    for j in (0..HT).rev() {
        for i in 0..WD {
            let color = ray_tracer.color_pixel(&scene, &config, i, j);

            let r = (255.99 * color.x) as u8;
            let g = (255.99 * color.y) as u8;
            let b = (255.99 * color.z) as u8;

            println!("{} {} {}", r, g, b);
        }
    }
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
        ],
    }
}
