mod dielectric;
mod lambertian;
mod light;
mod metal;
mod util;

use crate::cache::Cache;
use crate::geometry::HitInfo;
use crate::primitive::Ray3;
use crate::texture::Texture;

pub use dielectric::Dielectric;
use glam::Vec3;
pub use lambertian::Lambertian;
pub use light::Light;
pub use metal::Metal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Material {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Light(Light),
    Metal(Metal),
}

pub enum Interaction {
    NonTerminal { ray: Ray3, attenuation: Vec3 },
    Terminal { color: Vec3 },
}

impl Material {
    pub fn interact(
        &self,
        texture_cache: &Cache<Texture>,
        ray: &Ray3,
        hit: &HitInfo,
    ) -> Interaction {
        match self {
            Material::Dielectric(mat) => mat.interact(texture_cache, ray, hit),
            Material::Lambertian(mat) => mat.interact(texture_cache, hit),
            Material::Light(mat) => mat.interact(texture_cache, hit),
            Material::Metal(mat) => mat.interact(texture_cache, ray, hit),
        }
    }
}
