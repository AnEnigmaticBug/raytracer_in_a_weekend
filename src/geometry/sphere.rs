use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::bvh::Aabb;
use crate::primitive::Ray3;
use crate::util::compute_uv_on_sphere_from_normal;

use super::{HitInfo, Tbn3};

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        let oc = ray.pos - self.center;

        let a = ray.dir.length_squared();
        let b = ray.dir.dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;

        if discriminant > 0.0 {
            for &t in [
                (-b - (b.powi(2) - a * c).sqrt()) / a,
                (-b + (b.powi(2) - a * c).sqrt()) / a,
            ]
            .iter()
            {
                if tmin < t && t < tmax {
                    let pos = ray.point_at_param(t);
                    let normal = (pos - self.center) / self.radius;
                    let (u, v) = compute_uv_on_sphere_from_normal(normal);
                    return Some(HitInfo {
                        t,
                        u,
                        v,
                        pos,
                        tbn: Tbn3::from_tn(Vec3::Y.cross(normal), normal),
                    });
                }
            }
        }

        None
    }

    pub fn aabb(&self) -> Aabb {
        let radius = Vec3::splat(self.radius);
        Aabb {
            min: self.center - radius,
            max: self.center + radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::{geometry::Tbn3, primitive::Ray3};

    use super::*;

    #[test]
    fn sphere_hit_values_are_correct_along_the_equator() {
        let sphere = Sphere {
            center: Vec3::ZERO,
            radius: 1.0,
        };

        // hit on +z axis
        let ray = Ray3::new(2.0 * Vec3::Z, -Vec3::Z);
        let hit = sphere.hit(&ray, 0.0, 2.0).unwrap();

        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.pos, Vec3::Z);
        assert_eq!(hit.tbn, Tbn3::from_tn(Vec3::X, Vec3::Z));

        // hit on -x axis
        let ray = Ray3::new(-2.0 * Vec3::X, Vec3::X);
        let hit = sphere.hit(&ray, 0.0, 2.0).unwrap();

        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.pos, -Vec3::X);
        assert_eq!(hit.tbn, Tbn3::from_tn(Vec3::Z, -Vec3::X));
    }
}
