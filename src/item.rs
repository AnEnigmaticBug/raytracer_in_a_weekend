use serde::{Deserialize, Serialize};

use crate::geometry::{Geometry, HitInfo};
use crate::material::Material;
use crate::primitive::Ray3;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub geometry: Geometry,
    pub material: Material,
}

pub struct HitInfoAndMaterial<'a>(pub HitInfo, pub &'a Material);

impl Item {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfoAndMaterial> {
        self.geometry
            .hit(ray, tmin, tmax)
            .map(|hit_info| HitInfoAndMaterial(hit_info, &self.material))
    }
}
