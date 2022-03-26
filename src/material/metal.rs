use serde::{Deserialize, Serialize};

use crate::geometry::HitInfo;
use crate::primitive::{Ray3, Vec3};
use crate::texture::Texture;

use super::{util::rand_pos_in_sphere, Interaction};

#[derive(Serialize, Deserialize)]
pub struct Metal {
    pub texture: Texture,
    pub fuzz: f32,
}

impl Metal {
    pub fn interact(&self, ray: &Ray3, hit: &HitInfo) -> Interaction {
        let reflected_dir = ray.dir.normalized().reflect(&hit.normal);
        let scattered_ray = Ray3::new(hit.pos, reflected_dir + rand_pos_in_sphere(self.fuzz));

        if scattered_ray.dir.dot(&hit.normal) > 0.0 {
            Interaction::NonTerminal {
                ray: scattered_ray,
                attenuation: self.texture.color(hit.u, hit.v),
            }
        } else {
            Interaction::Terminal {
                color: Vec3::all(0.0),
            }
        }
    }
}
