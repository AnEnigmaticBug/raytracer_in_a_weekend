use serde::{Deserialize, Serialize};

use crate::material::Material;
use crate::primitive::{Ray3, Vec3};

use super::HitInfo;

#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub center: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        let normal = self.u.cross(&self.v);

        let num = (self.center - ray.pos).dot(&normal);
        let den = ray.dir.dot(&normal);

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

        if self.u.dot(&sep).abs() > self.u.len_squared()
            || self.v.dot(&sep).abs() > self.v.len_squared()
        {
            // Ray never touches this plane
            return None;
        }

        Some(HitInfo {
            t,
            pos,
            normal,
            material: &self.material,
        })
    }
}
