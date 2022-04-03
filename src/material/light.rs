use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::geometry::HitInfo;
use crate::texture::Texture;

use super::Interaction;

#[derive(Serialize, Deserialize)]
pub struct Light {
    pub texture: Texture,
    pub brightness: Vec3,
}

impl Light {
    pub fn interact(&self, hit: &HitInfo) -> Interaction {
        Interaction::Terminal {
            color: self.texture.color(hit.u, hit.v) * self.brightness,
        }
    }
}
