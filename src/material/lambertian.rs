use crate::geometry::HitInfo;
use crate::primitive::{Ray3, Vec3};

use super::util::rand_pos_in_sphere;
use super::RayInfo;

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn interact(&self, hit: &HitInfo) -> Option<RayInfo> {
        let target = hit.pos + hit.normal + rand_pos_in_sphere(1.0);
        Some(RayInfo {
            ray: Ray3::new(hit.pos, target - hit.pos),
            attenuation: self.albedo,
        })
    }
}
