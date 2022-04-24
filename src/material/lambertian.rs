use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::geometry::HitInfo;
use crate::primitive::Ray3;
use crate::texture::Texture;

use super::util::rand_pos_in_sphere;
use super::Interaction;

#[derive(Serialize, Deserialize)]
pub struct Lambertian {
    pub texture_idx: usize,
    pub normal_map_idx: Option<usize>,
}

impl Lambertian {
    pub fn interact(&self, texture_cache: &Cache<Texture>, hit: &HitInfo) -> Interaction {
        let normal = self
            .normal_map_idx
            .map(|idx| texture_cache[idx].normal(hit.u, hit.v, hit.tbn.matrix()))
            .unwrap_or(hit.tbn.n);

        let target = hit.pos + normal + rand_pos_in_sphere(1.0);
        Interaction::NonTerminal {
            ray: Ray3::new(hit.pos, target - hit.pos),
            attenuation: texture_cache[self.texture_idx].color(hit.u, hit.v),
        }
    }
}
