use crate::camera::Camera;
use crate::geometry::{Geometry, HitInfo};
use crate::primitive::{Ray3, Vec3};

pub struct Scene {
    pub sky_color: Vec3,
    pub camera: Camera,
    pub items: Vec<Geometry>,
}

impl Scene {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        let mut closest_hit: Option<HitInfo> = None;

        for hit in self
            .items
            .iter()
            .filter_map(|item| item.hit(ray, tmin, tmax))
        {
            closest_hit = match closest_hit {
                Some(closest_hit) if hit.t < closest_hit.t => Some(hit),
                None => Some(hit),
                _ => closest_hit,
            };
        }

        closest_hit
    }
}
