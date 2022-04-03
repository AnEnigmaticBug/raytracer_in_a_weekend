use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::bvh::Aabb;
use crate::primitive::Ray3;

use super::HitInfo;

#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub center: Vec3,
    pub u: Vec3,
    pub v: Vec3,
}

impl Plane {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        let normal = self.u.cross(self.v);

        let num = (self.center - ray.pos).dot(normal);
        let den = ray.dir.dot(normal);

        if den.abs() < 0.001 {
            // Ray is parallel to the plane
            return None;
        }

        let t = num / den;

        if t < tmin || t > tmax {
            return None;
        }

        let pos = ray.point_at_param(t);
        let sep = pos - self.center;

        if self.u.dot(sep).abs() > self.u.length_squared()
            || self.v.dot(sep).abs() > self.v.length_squared()
        {
            // Ray never touches this plane
            return None;
        }

        todo!("Add UV calculation")
    }

    pub fn aabb(&self) -> Aabb {
        todo!("Add AABB computation logic")
    }
}
