mod plane;
mod sphere;

use crate::bvh::Aabb;
use crate::primitive::{Ray3, Vec3};

pub use plane::Plane;
use serde::{Deserialize, Serialize};
pub use sphere::Sphere;

#[derive(Serialize, Deserialize)]
pub enum Geometry {
    Plane(Plane),
    Sphere(Sphere),
}

pub struct HitInfo {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub pos: Vec3,
    pub normal: Vec3,
}

impl Geometry {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        match self {
            Geometry::Plane(plane) => plane.hit(ray, tmin, tmax),
            Geometry::Sphere(sphere) => sphere.hit(ray, tmin, tmax),
        }
    }

    fn aabb(&self) -> Aabb {
        match self {
            Geometry::Plane(plane) => plane.aabb(),
            Geometry::Sphere(sphere) => sphere.aabb(),
        }
    }
}
