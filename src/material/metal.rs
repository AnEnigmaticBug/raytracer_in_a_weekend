use crate::geometry::HitInfo;
use crate::primitive::{Ray3, Vec3};

use super::{util::rand_pos_in_sphere, RayInfo};

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn interact(&self, ray: &Ray3, hit: &HitInfo) -> Option<RayInfo> {
        let reflected_dir = ray.dir.normalized().reflect(&hit.normal);
        let scattered_ray = Ray3::new(hit.pos, reflected_dir + rand_pos_in_sphere(self.fuzz));

        if scattered_ray.dir.dot(&hit.normal) > 0.0 {
            Some(RayInfo {
                ray: scattered_ray,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
