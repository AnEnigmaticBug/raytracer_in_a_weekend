use std::f32::consts::PI;

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::bvh::Aabb;
use crate::primitive::Ray3;

use super::HitInfo;

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        let oc = ray.pos - self.center;

        let a = ray.dir.length_squared();
        let b = ray.dir.dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            for &t in [
                (-b - (b.powi(2) - a * c).sqrt()) / a,
                (-b + (b.powi(2) - a * c).sqrt()) / a,
            ]
            .iter()
            {
                if tmin < t && t < tmax {
                    let pos = ray.point_at_param(t);
                    let normal = (pos - self.center) / self.radius;
                    return Some(HitInfo {
                        t,
                        u: 0.5 - (normal.z / normal.x).atan() / PI,
                        v: 1.0 - (normal.y + 1.0) / 2.0,
                        pos,
                        normal,
                    });
                }
            }
        }

        None
    }

    pub fn aabb(&self) -> Aabb {
        let radius = Vec3::splat(self.radius);
        Aabb {
            min: self.center - radius,
            max: self.center + radius,
        }
    }
}
