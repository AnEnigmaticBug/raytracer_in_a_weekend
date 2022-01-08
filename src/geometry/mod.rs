mod plane;
mod sphere;

use crate::material::Material;
use crate::primitive::{Ray3, Vec3};

pub use plane::Plane;
pub use sphere::Sphere;

pub enum Geometry {
    Plane(Plane),
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
            Geometry::Plane(plane) => plane.hit(ray, tmin, tmax),
            Geometry::Sphere(sphere) => sphere.hit(ray, tmin, tmax),
        }
    }
}
