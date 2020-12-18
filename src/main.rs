use raytracer::{
    camera::Camera,
    primitive::Vec3,
    ray_tracer::{Config, RayTracer},
};

const WD: u32 = 200;
const HT: u32 = 100;

fn main() {
    let ray_tracer = RayTracer::new();
    let config = Config {
        canvas_wd: WD,
        canvas_ht: HT,
        sky_color: Vec3::new(0.5, 0.7, 1.0),
        camera: Camera::new(Vec3::all(0.0), 90.0, WD as f32 / HT as f32),
    };

    println!("P3\n{} {}\n255", WD, HT);

    for j in (0..HT).rev() {
        for i in 0..WD {
            let color = ray_tracer.color_pixel(&config, i, j);

            let r = (255.99 * color.x) as u8;
            let g = (255.99 * color.y) as u8;
            let b = (255.99 * color.z) as u8;

            println!("{} {} {}", r, g, b);
        }
    }
}
