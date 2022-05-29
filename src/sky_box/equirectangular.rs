use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::texture::Texture;
use crate::util::compute_uv_on_sphere_from_normal;

#[derive(Serialize, Deserialize)]
pub struct Equirectangular {
    pub tex_idx: usize,
}

impl Equirectangular {
    pub fn color(&self, texture_cache: &Cache<Texture>, dir: Vec3) -> Vec3 {
        let (u, v) = compute_uv_on_sphere_from_normal(dir);
        texture_cache[self.tex_idx].color(u, v)
    }
}
