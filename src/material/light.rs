use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::geometry::HitInfo;
use crate::texture::Texture;

use super::Interaction;

#[derive(Serialize, Deserialize)]
pub struct Light {
    pub texture_idx: usize,
    pub brightness: Vec3,
}

impl Light {
    pub fn interact(&self, texture_cache: &Cache<Texture>, hit: &HitInfo) -> Interaction {
        Interaction::Terminal {
            color: texture_cache[self.texture_idx].color(hit.u, hit.v) * self.brightness,
        }
    }
}
