use clap::Parser;
use raytracer::{ray_tracer::RayTracer, scene::Scene};

/// Read a scene description from a JSON file and ray trace it.
#[derive(Parser)]
#[clap(about)]
struct CliArgs {
    #[clap(flatten)]
    ray_tracer: RayTracer,
    /// The JSON file which contains the scene description.
    #[clap(long)]
    scene: String,
    /// The desired path of the rendered image. The extension (png/jpg) decides
    /// the image format.
    #[clap(long, default_value = "scene.png")]
    output: String,
}

fn main() {
    let args = CliArgs::parse();
    let ray_tracer = args.ray_tracer;
    let scene = Scene::from_json(args.scene).expect("Couldn't read scene");

    ray_tracer
        .render_to_file(&scene, args.output)
        .expect("Couldn't write image data");
}
