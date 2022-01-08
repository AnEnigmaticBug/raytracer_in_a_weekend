![Screenshot](screenshot.png "This is has been resized to 50% to reduce size. The original looks pretty sharp.")

`raytracer_in_a_weekend` is my attempt at making a ray-tracer while following along the excellent _Ray Tracing in One Weekend_ guide. It also served as a good mini-project to brush up my Rust skills.

The provided demo (in [random.rs](src/bin/random.rs)) renders a randomly generated scene. The scene consists of 2 planes + a collection of spheres with one of 3 types of materials:

- Lambertian (matte)
- Metal
- Dielectric (glass)

While the raytracer in _Ray Tracing in One Weekend_ is single-threaded, I've used `rayon` for multi-threading. I've also added support for planes.

## Running

1. Ensure you've setup `cargo` properly.
2. `cd` to the project's root directory.
3. You have 2 options:
    - `cargo run --release --bin random` will render a randomized scene full of spheres
    - `cargo run --release --bin custom` will render a scene as per `inputs/scene.json` (you can edit this file)
    
    Rendering is CPU intensive. To speed things up, `rayon` will try to use as many CPU cores as possible. A progress bar will be shown to ensure that you're not left waiting blindly.
4. The rendered scene will be saved as `scene.png`.
