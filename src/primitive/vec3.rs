use glam::Vec3;

pub trait Vec3Utils {
    fn reflect(&self, normal: Self) -> Self;
    fn refract(&self, normal: Self, ni_by_nt: f32) -> Option<Self>
    where
        Self: Sized;
}

impl Vec3Utils for Vec3 {
    fn reflect(&self, normal: Self) -> Self {
        let normal_component = self.dot(normal) * normal;
        *self - 2.0 * normal_component
    }

    fn refract(&self, normal: Self, ni_by_nt: f32) -> Option<Self> {
        let dir = self.normalize();
        let cos = dir.dot(normal);
        let discriminant = 1.0 - ni_by_nt.powi(2) * (1.0 - cos.powi(2));

        if discriminant > 0.0 {
            Some(ni_by_nt * (dir - normal * cos) - normal * discriminant.sqrt())
        } else {
            // Total internal reflection
            None
        }
    }
}
