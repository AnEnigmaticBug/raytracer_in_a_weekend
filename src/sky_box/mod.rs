mod cubemap;
mod equirectangular;

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::texture::Texture;

pub use cubemap::Cubemap;
pub use equirectangular::Equirectangular;

#[derive(Serialize, Deserialize)]
pub enum SkyBox {
    Cubemap(Cubemap),
    Equirectangular(Equirectangular),
}

impl SkyBox {
    pub fn color(&self, texture_cache: &Cache<Texture>, dir: Vec3) -> Vec3 {
        match self {
            Self::Cubemap(skybox) => skybox.color(texture_cache, dir),
            Self::Equirectangular(skybox) => skybox.color(texture_cache, dir),
        }
    }
}
