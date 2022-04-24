use glam::{Mat3, Vec3};
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

    /// Assumes that the texture represents a normal map and returns the normal
    /// corresponding to the passed uv coordinates _in the scene space_.
    ///
    /// It takes in a change of basis matrix from TBN to scene space.
    pub fn normal(&self, u: f32, v: f32, tbn: Mat3) -> Vec3 {
        // RGB values are in [0, 1]. Vectors are in [-1, 1].
        let tbn_space_normal = self.color(u, v) * 2.0 - 1.0;
        (tbn * tbn_space_normal).normalize()
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::geometry::Tbn3;

    use super::*;

    fn always_z_normal_map() -> Texture {
        Texture::Solid(Solid {
            color: (Vec3::Z + 1.0) / 2.0,
        })
    }

    /// We take a normal map with all its normals pointing in the z direction
    /// and apply it to a sphere.
    ///
    /// The new normals will match the old ones. If a point on the sphere had
    /// a normal in direction 'd' earlier, it will still have the same normal.
    #[test]
    fn normal_on_a_sphere_is_unchanged() {
        let normal_map = always_z_normal_map();

        // point facing us (normal along pos z-axis)
        let normal = Vec3::Z;
        let tbn = Tbn3::from_tn(Vec3::X, normal);
        assert_eq!(normal, normal_map.normal(0.50, 0.5, tbn.matrix()));

        // point in the back (normal along neg z-axis)
        let normal = -Vec3::Z;
        let tbn = Tbn3::from_tn(-Vec3::X, normal);
        assert_eq!(normal, normal_map.normal(0.00, 0.5, tbn.matrix()));

        // point on the right side (normal along pos x-axis)
        let normal = Vec3::X;
        let tbn = Tbn3::from_tn(-Vec3::Z, normal);
        assert_eq!(normal, normal_map.normal(0.75, 0.5, tbn.matrix()));

        // point on the left side (normal along neg x-axis)
        let normal = -Vec3::X;
        let tbn = Tbn3::from_tn(Vec3::Z, normal);
        assert_eq!(normal, normal_map.normal(0.25, 0.5, tbn.matrix()));
    }
}
