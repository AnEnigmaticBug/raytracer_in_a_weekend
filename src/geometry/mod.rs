mod scene;
mod sphere;

use crate::material::Material;
use crate::primitive::{Ray3, Vec3};

pub use scene::Scene;
pub use sphere::Sphere;

pub enum Geometry {
    Sphere(Sphere),
}

pub struct HitInfo<'a> {
    pub t: f32,
    pub pos: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

impl Geometry {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        match self {
            Geometry::Sphere(sphere) => sphere.hit(ray, tmin, tmax),
        }
    }
}
