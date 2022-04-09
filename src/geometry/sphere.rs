use std::f32::consts::{PI, TAU};

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::bvh::Aabb;
use crate::primitive::Ray3;

use super::HitInfo;

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
                    let (u, v) = compute_uv(normal);
                    return Some(HitInfo {
                        t,
                        u,
                        v,
                        pos,
                        normal,
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

/// Computes the UV coordinates for a given normal on a sphere.
///
/// u starts from 0 at -z axis and increases as we move anti clockwise (viewed
/// from the top) till it reaches -z axis again. For example:
/// * -x axis is 0.25,
/// * +z axis is 0.50,
/// * +x axis is 0.75 and so on.
///
/// v goes from 0 at north pole to 1 at the south pole.
fn compute_uv(normal: Vec3) -> (f32, f32) {
    // The angle is measured from the -z axis in the anti clockwise direction.
    // (viewed from the top). It goes from 0 to 2π.
    let angle = f32::atan2(normal.x, normal.z) + PI;
    (angle / TAU, 1.0 - (normal.y + 1.0) / 2.0)
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::primitive::Ray3;

    use super::*;

    #[test]
    fn compute_uv_works_for_equatorial_points() {
        assert_eq!((0.00, 0.5), compute_uv(-Vec3::Z));
        assert_eq!((0.25, 0.5), compute_uv(-Vec3::X));
        assert_eq!((0.50, 0.5), compute_uv(Vec3::Z));
        assert_eq!((0.75, 0.5), compute_uv(Vec3::X));
    }

    #[test]
    fn compute_uv_works_for_poles() {
        assert_eq!(0.0, compute_uv(Vec3::Y).1);
        assert_eq!(1.0, compute_uv(-Vec3::Y).1);
    }

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

        // hit on -x axis
        let ray = Ray3::new(-2.0 * Vec3::X, Vec3::X);
        let hit = sphere.hit(&ray, 0.0, 2.0).unwrap();

        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.pos, -Vec3::X);
    }
}
