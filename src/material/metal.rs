use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::geometry::HitInfo;
use crate::primitive::{Ray3, Vec3Utils};
use crate::texture::Texture;

use super::{util::rand_pos_in_sphere, Interaction};

#[derive(Serialize, Deserialize)]
pub struct Metal {
    pub texture_idx: usize,
    pub normal_map_idx: Option<usize>,
    pub fuzz: f32,
}

impl Metal {
    pub fn interact(
        &self,
        texture_cache: &Cache<Texture>,
        ray: &Ray3,
        hit: &HitInfo,
    ) -> Interaction {
        let normal = self
            .normal_map_idx
            .map(|idx| texture_cache[idx].normal(hit.u, hit.v, hit.tbn.matrix()))
            .unwrap_or(hit.tbn.n);

        let reflected_dir = ray.dir.normalize().reflect(normal);
        let scattered_ray = Ray3::new(hit.pos, reflected_dir + rand_pos_in_sphere(self.fuzz));

        if scattered_ray.dir.dot(normal) > 0.0 {
            Interaction::NonTerminal {
                ray: scattered_ray,
                attenuation: texture_cache[self.texture_idx].color(hit.u, hit.v),
            }
        } else {
            Interaction::Terminal { color: Vec3::ZERO }
        }
    }
}
