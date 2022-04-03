use glam::Vec3;
use rand::Rng;

pub(super) fn rand_pos_in_sphere(radius: f32) -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let pos = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::ONE;

        if pos.length_squared() < 1.0 {
            return pos * radius;
        }
    }
}
