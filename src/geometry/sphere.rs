use serde::{Deserialize, Serialize};

use crate::material::Material;
use crate::primitive::{Ray3, Vec3};

use super::HitInfo;

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        let oc = ray.pos - self.center;

        let a = ray.dir.len_squared();
        let b = ray.dir.dot(&oc);
        let c = oc.len_squared() - self.radius.powi(2);

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
                    return Some(HitInfo {
                        t,
                        pos: pos,
                        normal: (pos - self.center) / self.radius,
                        material: &self.material,
                    });
                }
            }
        }

        None
    }
}
