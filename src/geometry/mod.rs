mod plane;
mod sphere;

use crate::bvh::Aabb;
use crate::primitive::Ray3;

use glam::{Mat3, Vec3};
pub use plane::Plane;
use serde::{Deserialize, Serialize};
pub use sphere::Sphere;

#[derive(Serialize, Deserialize)]
pub enum Geometry {
    Plane(Plane),
    Sphere(Sphere),
}

/// A struct for holding the tangent, bitangent, and the normal vectors. These
/// are often used together in normal maps.
#[derive(Clone, Debug, PartialEq)]
pub struct Tbn3 {
    /// The tangent vector of the TBN space. It's equivalent to the positive x
    /// vector in the TBN space.
    pub t: Vec3,
    /// The bitangent vector of the TBN space. It's equivalent to the positive
    /// y vector in the TBN space.
    pub b: Vec3,
    /// The normal vector of the TBN space. It is equivalent to the positive z
    /// vector in the TBN space.
    pub n: Vec3,
}

impl Tbn3 {
    pub fn from_tn(t: Vec3, n: Vec3) -> Self {
        Self {
            t: t.normalize(),
            b: n.cross(t).normalize(),
            n: n.normalize(),
        }
    }

    /// Returns the change of basis matrix from TBN space to the scene space.
    pub fn matrix(&self) -> Mat3 {
        Mat3::from_cols(self.t, self.b, self.n)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HitInfo {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub pos: Vec3,
    pub tbn: Tbn3,
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
