use clap::Parser;
use glam::Vec3;
use rand::{thread_rng, Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use raytracer::{
    cache::Cache,
    camera::CameraInitOptions,
    geometry::{Geometry, Sphere},
    item::Item,
    material::{Dielectric, Lambertian, Material, Metal},
    scene::Scene,
    sky_box::SkyBox,
    texture::{Image, Solid, Texture},
};

/// Generate a scene made of randomly placed balls and ray trace it.
#[derive(Parser)]
#[clap(about)]
struct CliArgs {
    /// Aspect ratio of the camera.
    #[clap(long, default_value_t = 2.00)]
    aspect: f32,
    /// The seed of the RNG which places items in the scene. The same seed will
    /// result in the same scene. If you don't provide a seed, one will created
    /// for you.
    #[clap(long)]
    seed: Option<u64>,
}

fn main() {
    let args = CliArgs::parse();
    let seed = args.seed.unwrap_or(thread_rng().gen());
    eprintln!("{} is the scene generation seed", seed);
    let scene = setup_scene(seed, args.aspect);
    let json = serde_json::to_string_pretty(&scene).expect("Couldn't serialize scene");
    println!("{}", json);
}

fn setup_scene(scene_seed: u64, aspect: f32) -> Scene {
    let mut texture_cache = Cache::new();

    let mut scene = Scene {
        sky_box: SkyBox {
            up_idx: texture_cache.add(
                "skybox_up",
                Texture::Image(
                    Image::load("inputs/textures/yellowcloud_up.png")
                        .expect("Couldn't load texture"),
                ),
            ),
            dn_idx: texture_cache.add(
                "skybox_dn",
                Texture::Image(
                    Image::load("inputs/textures/yellowcloud_dn.png")
                        .expect("Couldn't load texture"),
                ),
            ),
            lf_idx: texture_cache.add(
                "skybox_lf",
                Texture::Image(
                    Image::load("inputs/textures/yellowcloud_lf.png")
                        .expect("Couldn't load texture"),
                ),
            ),
            rt_idx: texture_cache.add(
                "skybox_rt",
                Texture::Image(
                    Image::load("inputs/textures/yellowcloud_rt.png")
                        .expect("Couldn't load texture"),
                ),
            ),
            ft_idx: texture_cache.add(
                "skybox_ft",
                Texture::Image(
                    Image::load("inputs/textures/yellowcloud_ft.png")
                        .expect("Couldn't load texture"),
                ),
            ),
            bk_idx: texture_cache.add(
                "skybox_bk",
                Texture::Image(
                    Image::load("inputs/textures/yellowcloud_bk.png")
                        .expect("Couldn't load texture"),
                ),
            ),
        },
        camera: CameraInitOptions {
            pos: Vec3::new(3.0, 1.5, 8.0),
            look_at: Vec3::new(0.5, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            vt_fov: 30.0,
            aspect,
        }
        .into(),
        texture_cache,
        geometry_cache: Cache::new(),
        material_cache: Cache::new(),
        items: Vec::with_capacity(1 + 12 * 12 + 3),
    };

    scene.items.push(Item {
        geometry_idx: scene.geometry_cache.add(
            "ground",
            Geometry::Sphere(Sphere {
                center: Vec3::new(0.0, -1000.0, 0.0),
                radius: 1000.0,
            }),
        ),
        material_idx: scene.material_cache.add(
            "ground",
            Material::Lambertian(Lambertian {
                texture_idx: scene.texture_cache.add(
                    "gray",
                    Texture::Solid(Solid {
                        color: Vec3::splat(0.5),
                    }),
                ),
            }),
        ),
    });

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(scene_seed);

    for a in -6..6 {
        for b in -6..6 {
            let offset = Vec3::new(rng.gen(), 0.0, rng.gen()) * 0.6;
            let center = Vec3::new(a as f32, 0.2, b as f32) + offset;

            let material_chooser = rng.gen::<f32>();
            let material = if material_chooser < 0.8 {
                Material::Lambertian(Lambertian {
                    texture_idx: scene.texture_cache.add(
                        format!("rand_tex_{}_{}", a, b),
                        Texture::Solid(Solid {
                            color: Vec3::new(
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                                rng.gen::<f32>() * rng.gen::<f32>(),
                            ),
                        }),
                    ),
                })
            } else if material_chooser < 0.95 {
                Material::Metal(Metal {
                    texture_idx: scene.texture_cache.add(
                        format!("rand_tex_{}_{}", a, b),
                        Texture::Solid(Solid {
                            color: Vec3::new(
                                0.5 * (rng.gen::<f32>() + 1.0),
                                0.5 * (rng.gen::<f32>() + 1.0),
                                0.5 * (rng.gen::<f32>() + 1.0),
                            ),
                        }),
                    ),
                    fuzz: 0.5 * rng.gen::<f32>(),
                })
            } else {
                Material::Dielectric(Dielectric { ref_idx: 1.5 })
            };

            scene.items.push(Item {
                geometry_idx: scene.geometry_cache.add(
                    format!("rand_geo_{}_{}", a, b),
                    Geometry::Sphere(Sphere {
                        center,
                        radius: 0.2,
                    }),
                ),
                material_idx: scene
                    .material_cache
                    .add(format!("rand_mat_{}_{}", a, b), material),
            });
        }
    }

    scene.items.push(Item {
        geometry_idx: scene.geometry_cache.add(
            "lambertian_main",
            Geometry::Sphere(Sphere {
                center: Vec3::new(-1.0, 1.0, -1.5),
                radius: 1.0,
            }),
        ),
        material_idx: scene.material_cache.add(
            "lambertian_main",
            Material::Lambertian(Lambertian {
                texture_idx: scene.texture_cache.add(
                    "lambertian_main",
                    Texture::Solid(Solid {
                        color: Vec3::new(0.4, 0.2, 0.1),
                    }),
                ),
            }),
        ),
    });

    scene.items.push(Item {
        geometry_idx: scene.geometry_cache.add(
            "glass_main",
            Geometry::Sphere(Sphere {
                center: Vec3::new(0.0, 1.0, 0.0),
                radius: 1.0,
            }),
        ),
        material_idx: scene.material_cache.add(
            "glass_main",
            Material::Dielectric(Dielectric { ref_idx: 1.5 }),
        ),
    });

    scene.items.push(Item {
        geometry_idx: scene.geometry_cache.add(
            "metal_main",
            Geometry::Sphere(Sphere {
                center: Vec3::new(1.0, 1.0, 1.5),
                radius: 1.0,
            }),
        ),
        material_idx: scene.material_cache.add(
            "metal_main",
            Material::Metal(Metal {
                texture_idx: scene.texture_cache.add(
                    "metal_main",
                    Texture::Solid(Solid {
                        color: Vec3::new(0.7, 0.6, 0.5),
                    }),
                ),
                fuzz: 0.0,
            }),
        ),
    });

    scene
}
