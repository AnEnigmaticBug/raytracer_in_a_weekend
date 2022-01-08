use rand::random;
use serde::{Deserialize, Serialize};

use crate::geometry::HitInfo;
use crate::primitive::{Ray3, Vec3};

use super::RayInfo;

#[derive(Serialize, Deserialize)]
pub struct Dielectric {
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn interact(&self, ray: &Ray3, hit: &HitInfo) -> Option<RayInfo> {
        let outward_normal;
        let ni_by_nt;
        let cos;

        if ray.dir.dot(&hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_by_nt = self.ref_idx;
            cos = self.ref_idx * ray.dir.normalized().dot(&hit.normal);
        } else {
            outward_normal = hit.normal;
            ni_by_nt = 1.0 / self.ref_idx;
            cos = -ray.dir.normalized().dot(&hit.normal);
        }

        if let Some(refraction_dir) = ray.dir.refract(&outward_normal, ni_by_nt) {
            let reflection_probability = schlick(cos, self.ref_idx);

            if random::<f32>() > reflection_probability {
                return Some(RayInfo {
                    ray: Ray3::new(hit.pos, refraction_dir),
                    attenuation: Vec3::all(1.0),
                });
            }
        }

        let reflection_dir = ray.dir.reflect(&hit.normal);

        Some(RayInfo {
            ray: Ray3::new(hit.pos, reflection_dir),
            attenuation: Vec3::all(1.0),
        })
    }
}

fn schlick(cos: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
