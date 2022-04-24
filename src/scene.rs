use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::camera::Camera;
use crate::geometry::{Geometry, HitInfo};
use crate::item::Item;
use crate::material::Material;
use crate::primitive::Ray3;
use crate::sky_box::SkyBox;
use crate::texture::Texture;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub sky_box: SkyBox,
    pub camera: Camera,
    pub texture_cache: Cache<Texture>,
    pub geometry_cache: Cache<Geometry>,
    pub material_cache: Cache<Material>,
    pub items: Vec<Item>,
}

pub struct HitInfoAndMaterial<'a>(pub HitInfo, pub &'a Material);

impl Scene {
    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(serde_json::from_str(&contents)?)
    }

    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfoAndMaterial> {
        let mut closest_hit: Option<HitInfoAndMaterial> = None;

        for hit in self
            .items
            .iter()
            .filter_map(|item| self.hit_item(item, ray, tmin, tmax))
        {
            closest_hit = match closest_hit {
                Some(closest_hit) if hit.0.t < closest_hit.0.t => Some(hit),
                None => Some(hit),
                _ => closest_hit,
            };
        }

        closest_hit
    }

    fn hit_item(
        &self,
        item: &Item,
        ray: &Ray3,
        tmin: f32,
        tmax: f32,
    ) -> Option<HitInfoAndMaterial> {
        self.geometry_cache[item.geometry_idx]
            .hit(ray, tmin, tmax)
            .map(|hit_info| HitInfoAndMaterial(hit_info, &self.material_cache[item.material_idx]))
    }
}
