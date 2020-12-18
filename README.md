![Screenshot](screenshot.png "This is has been resized to 50% to reduce size. The original looks pretty sharp.")

`raytracer_in_a_weekend` is my attempt at making a ray-tracer while following along the excellent _Ray Tracing in One Weekend_ guide. It also served as a good mini-project to brush up my Rust skills.

The provided demo (in [main.rs](src/main.rs)) renders a randomly generated scene. The scene consists of a collection of spheres with one of 3 types of materials:

- Lambertian (matte)
- Metal
- Dielectric (glass)

While the raytracer in _Ray Tracing in One Weekend_ is single-threaded, I've used `rayon` for multi-threading.

## Running

1. Ensure you've `cargo` setup properly.
2. Do `cargo run --release` in the project's root directory.
3. This will render the scene to `scene.png`. This part is CPU intensive. A progress bar is shown to ensure you're not left waiting blindly.
