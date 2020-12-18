use crate::geometry::HitInfo;
use crate::primitive::{Ray3, Vec3};

use super::RayInfo;

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn interact(&self, ray: &Ray3, hit: &HitInfo) -> Option<RayInfo> {
        let outward_normal;
        let ni_by_nt;

        if ray.dir.dot(&hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_by_nt = self.ref_idx;
        } else {
            outward_normal = hit.normal;
            ni_by_nt = 1.0 / self.ref_idx;
        }

        if let Some(refraction_dir) = ray.dir.refract(&outward_normal, ni_by_nt) {
            Some(RayInfo {
                ray: Ray3::new(hit.pos, refraction_dir),
                attenuation: Vec3::all(1.0),
            })
        } else {
            let reflection_dir = ray.dir.reflect(&hit.normal);

            Some(RayInfo {
                ray: Ray3::new(hit.pos, reflection_dir),
                attenuation: Vec3::all(1.0),
            })
        }
    }
}
