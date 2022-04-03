use glam::Vec3;

use crate::primitive::Ray3;

pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit(&self, ray: &Ray3, mut tmin: f32, mut tmax: f32) -> bool {
        for i in [0, 1, 2] {
            let t1 = (self.min[i] - ray.pos[i]) / ray.dir[i];
            let t2 = (self.max[i] - ray.pos[i]) / ray.dir[i];

            // t1 will be greater than t2 iff one of the following is true:
            // * ray.dir[i] is negative
            // * min[i] is greater than max[i]
            let intersection = intersect(t1.min(t2), t1.max(t2), tmin, tmax);
            tmin = intersection.0;
            tmax = intersection.1;

            if tmin > tmax {
                return false;
            }
        }
        true
    }
}

fn intersect(t1min: f32, t1max: f32, t2min: f32, t2max: f32) -> (f32, f32) {
    (t1min.max(t2min), t1max.min(t2max))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TMIN: f32 = 1.0;
    const TMAX: f32 = 5.0;

    #[test]
    fn intersect_works() {
        assert_eq!((2.5, 5.0), intersect(1.0, 5.0, 2.5, 6.0));

        let (min, max) = intersect(1.0, 2.0, 3.0, 4.0);
        assert!(min > max);
    }

    fn test_ray() -> Ray3 {
        Ray3::new(Vec3::ONE, Vec3::ONE)
    }

    #[test]
    fn aabb_hit_accepts_in_happy_case() {
        assert!(Aabb {
            min: Vec3::ONE,
            max: Vec3::splat(8.0),
        }
        .hit(&test_ray(), TMIN, TMAX));
    }

    #[test]
    fn aabb_hit_rejects_if_ray_dir_is_bad() {
        assert!(!Aabb {
            min: Vec3::new(0.0, 0.0, 3.0),
            max: Vec3::new(1.0, 1.0, 4.0),
        }
        .hit(&test_ray(), TMIN, TMAX));
    }

    #[test]
    fn aabb_hit_rejects_if_ray_dir_is_fine_but_ray_tmin_tmax_are_bad() {
        assert!(!Aabb {
            min: Vec3::splat(8.0),
            max: Vec3::splat(9.0),
        }
        .hit(&test_ray(), TMIN, TMAX));

        assert!(!Aabb {
            min: Vec3::ZERO,
            max: Vec3::ONE,
        }
        .hit(&test_ray(), TMIN, TMAX));
    }

    #[test]
    fn aabb_hit_works_when_ray_is_along_an_axis() {
        let ray = Ray3::new(Vec3::ONE, Vec3::new(1.0, 1.0, 0.0));

        assert!(Aabb {
            min: Vec3::ZERO,
            max: Vec3::splat(8.0),
        }
        .hit(&ray, TMIN, TMAX));

        assert!(!Aabb {
            min: Vec3::ZERO,
            max: Vec3::splat(0.5),
        }
        .hit(&ray, TMIN, TMAX));
    }

    #[test]
    fn aabb_hit_works_when_ray_has_negative_dir() {
        let ray = Ray3::new(Vec3::splat(7.0), Vec3::splat(-1.0));

        assert!(Aabb {
            min: Vec3::splat(2.0),
            max: Vec3::splat(8.0),
        }
        .hit(&ray, TMIN, TMAX));

        assert!(!Aabb {
            min: Vec3::ZERO,
            max: Vec3::splat(0.5),
        }
        .hit(&ray, TMIN, TMAX));
    }
}
