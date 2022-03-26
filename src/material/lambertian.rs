use serde::{Deserialize, Serialize};

use crate::geometry::HitInfo;
use crate::primitive::Ray3;
use crate::texture::Texture;

use super::util::rand_pos_in_sphere;
use super::Interaction;

#[derive(Serialize, Deserialize)]
pub struct Lambertian {
    pub texture: Texture,
}

impl Lambertian {
    pub fn interact(&self, hit: &HitInfo) -> Interaction {
        let target = hit.pos + hit.normal + rand_pos_in_sphere(1.0);
        Interaction::NonTerminal {
            ray: Ray3::new(hit.pos, target - hit.pos),
            attenuation: self.texture.color(hit.u, hit.v),
        }
    }
}
