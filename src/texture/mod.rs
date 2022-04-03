use glam::Vec3;
use serde::{Deserialize, Serialize};

pub use self::image::Image;
pub use self::linear_gradient::LinearGradient;
pub use self::solid::Solid;

mod image;
mod linear_gradient;
mod solid;

#[derive(Serialize, Deserialize)]
pub enum Texture {
    Image(Image),
    LinearGradient(LinearGradient),
    Solid(Solid),
}

impl Texture {
    pub fn color(&self, u: f32, v: f32) -> Vec3 {
        match self {
            Self::Image(image) => image.color(u, v),
            Self::LinearGradient(gradient) => gradient.color(u),
            Self::Solid(solid) => solid.color,
        }
    }
}
