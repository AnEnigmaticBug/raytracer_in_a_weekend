mod sphere;

use crate::primitive::{Ray3, Vec3};

pub use sphere::Sphere;

pub enum Geometry {
    Sphere(Sphere),
}

pub struct HitInfo {
    pub t: f32,
    pub pos: Vec3,
    pub normal: Vec3,
}

impl Geometry {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        match self {
            Geometry::Sphere(sphere) => sphere.hit(ray, tmin, tmax),
        }
    }
}
