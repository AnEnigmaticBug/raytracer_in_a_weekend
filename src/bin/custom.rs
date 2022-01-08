use raytracer::{ray_tracer::RayTracer, scene::Scene};

const WD: u32 = 512;
const HT: u32 = 256;

fn main() {
    let ray_tracer = RayTracer {
        canvas_wd: WD,
        canvas_ht: HT,
        num_samples: 16,
        max_reflections: 16,
    };
    let scene = Scene::from_json("inputs/scene.json").expect("Couldn't read scene");

    ray_tracer
        .render_to_file(&scene, "scene.png")
        .expect("Couldn't write image data");
}
