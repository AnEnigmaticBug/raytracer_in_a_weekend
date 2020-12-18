mod lambertian;
mod metal;
mod util;

use crate::geometry::HitInfo;
use crate::primitive::{Ray3, Vec3};

pub use lambertian::Lambertian;
pub use metal::Metal;

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

pub struct RayInfo {
    pub ray: Ray3,
    pub attenuation: Vec3,
}

impl Material {
    pub fn interact(&self, ray: &Ray3, hit: &HitInfo) -> Option<RayInfo> {
        match self {
            Material::Lambertian(mat) => mat.interact(hit),
            Material::Metal(mat) => mat.interact(ray, hit),
        }
    }
}
