use std::f32::consts::{FRAC_PI_2, PI, TAU};

use glam::Vec3;

pub fn map(val: f32, cur_min: f32, cur_max: f32, new_min: f32, new_max: f32) -> f32 {
    let percent = (val - cur_min) / (cur_max - cur_min);
    new_min + percent * (new_max - new_min)
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
pub fn compute_uv_on_sphere_from_normal(normal: Vec3) -> (f32, f32) {
    // The angle is measured from the -z axis in the anti clockwise direction.
    // (viewed from the top). It goes from 0 to 2π.
    let u_angle = f32::atan2(normal.x, normal.z) + PI;
    let v_angle = FRAC_PI_2 - normal.y.asin();
    (u_angle / TAU, v_angle / PI)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 0.001;

    #[test]
    fn map_happy_case() {
        assert!((map(0.0, -1.0, 1.0, 0.0, 1.0) - 0.5).abs() < EPS);
    }

    #[test]
    fn map_flip_range() {
        let lhs = map(0.0, 0.0, 1.0, 1.0, 0.0);
        let rhs = 1.0;
        assert!((lhs - rhs).abs() < EPS);
    }

    #[test]
    fn compute_uv_on_sphere_from_normal_works_for_equatorial_points() {
        assert_eq!((0.00, 0.5), compute_uv_on_sphere_from_normal(-Vec3::Z));
        assert_eq!((0.25, 0.5), compute_uv_on_sphere_from_normal(-Vec3::X));
        assert_eq!((0.50, 0.5), compute_uv_on_sphere_from_normal(Vec3::Z));
        assert_eq!((0.75, 0.5), compute_uv_on_sphere_from_normal(Vec3::X));
    }

    #[test]
    fn compute_uv_on_sphere_from_normal_works_for_poles() {
        assert_eq!(0.0, compute_uv_on_sphere_from_normal(Vec3::Y).1);
        assert_eq!(1.0, compute_uv_on_sphere_from_normal(-Vec3::Y).1);
    }
}
